use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io::{self, Write as IoWrite};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;

mod problem;
mod rendering;

use problem::heuristic;
use problem::heuristic::strategy::{
    marginal_envy, marginal_greedy, marginal_lazy, marginal_smart,
    marginal_wise, total_envy, total_greedy, total_lazy, total_smart,
    total_wise, Selector,
};
use problem::heuristic::ExperimentSpec;
use problem::Problem;
use rendering::point::Point;
use rendering::renderer::{Map, Renderer};

const DEFAULT_DATA_DIR: &str = "data";
const REPORTS_DIR: &str = "reports";
const DEFAULT_RUNS: usize = 1;

const START_NODE: u8 = 0;
const END_NODE: u8 = 1;

/// Randomized methods used in the final comparison reported in the article.
const RANDOMIZED_METHOD_NAMES: [&str; 4] = [
    "selected_random",
    "selected_weighted_random",
    "selected_random_random_tie",
    "random_feasible_insertion",
];

#[derive(Debug)]
struct MethodResult {
    method: String,
    runs: usize,
    unique_routes: usize,
    best_route: Vec<u8>,
    best_score: u32,
    best_cost: f32,
    best_found_at: usize,
    elapsed: Duration,
}

fn available_methods() -> Vec<ExperimentSpec> {
    let mut methods = vec![
        ExperimentSpec {
            name: "marginal_greedy".into(),
            selector: Selector::By(marginal_greedy),
        },
        ExperimentSpec {
            name: "marginal_lazy".into(),
            selector: Selector::By(marginal_lazy),
        },
        ExperimentSpec {
            name: "marginal_smart".into(),
            selector: Selector::By(marginal_smart),
        },
        ExperimentSpec {
            name: "marginal_wise".into(),
            selector: Selector::By(marginal_wise),
        },
        ExperimentSpec {
            name: "marginal_envy".into(),
            selector: Selector::By(marginal_envy),
        },
        ExperimentSpec {
            name: "total_greedy".into(),
            selector: Selector::By(total_greedy),
        },
        ExperimentSpec {
            name: "total_lazy".into(),
            selector: Selector::By(total_lazy),
        },
        ExperimentSpec {
            name: "total_smart".into(),
            selector: Selector::By(total_smart),
        },
        ExperimentSpec {
            name: "total_wise".into(),
            selector: Selector::By(total_wise),
        },
        ExperimentSpec {
            name: "total_envy".into(),
            selector: Selector::By(total_envy),
        },
    ];

    let randomized = heuristic::experiment_specs();

    for wanted_name in RANDOMIZED_METHOD_NAMES {
        let spec = randomized
            .iter()
            .find(|spec| spec.name == wanted_name)
            .unwrap_or_else(|| {
                panic!(
                    "The required randomized method {wanted_name:?} was not found"
                )
            });

        methods.push(spec.clone());
    }

    methods
}

fn collect_instance_files(
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> io::Result<()> {
    let mut entries = fs::read_dir(directory)?
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let file_type = entry.file_type()?;
        let path = entry.path();

        if file_type.is_dir() {
            collect_instance_files(&path, files)?;
        } else if file_type.is_file()
            && path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("txt"))
        {
            files.push(path);
        }
    }

    Ok(())
}

fn discover_instance_files(directory: &Path) -> io::Result<Vec<PathBuf>> {
    if !directory.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "data directory {:?} does not exist; run the program from the project root or pass the data-directory path as the first argument",
                directory
            ),
        ));
    }

    let mut files = Vec::new();
    collect_instance_files(directory, &mut files)?;
    files.sort();

    if files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("no .txt test instances were found inside {:?}", directory),
        ));
    }

    Ok(files)
}

fn prompt_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;

    if bytes_read == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "standard input was closed",
        ));
    }

    Ok(input.trim().to_owned())
}

fn choose_instance(
    data_directory: &Path,
    files: &[PathBuf],
) -> io::Result<PathBuf> {
    println!("\nAvailable test instances:\n");

    for (index, path) in files.iter().enumerate() {
        let displayed_path = path.strip_prefix(data_directory).unwrap_or(path);
        println!("{:>3}. {}", index + 1, displayed_path.display());
    }

    loop {
        let input = prompt_line("\nSelect an instance by number: ")?;

        if let Ok(number) = input.parse::<usize>() {
            if (1..=files.len()).contains(&number) {
                return Ok(files[number - 1].clone());
            }
        }

        eprintln!("Please enter a number between 1 and {}.", files.len());
    }
}

/// `None` means that every displayed method was selected.
fn choose_method(methods: &[ExperimentSpec]) -> io::Result<Option<usize>> {
    println!("\nAvailable methods:\n");
    println!("  0. all methods");

    for (index, method) in methods.iter().enumerate() {
        println!("{:>3}. {}", index + 1, method.name);
    }

    loop {
        let input = prompt_line("\nSelect a method by number: ")?;

        if let Ok(number) = input.parse::<usize>() {
            if number == 0 {
                return Ok(None);
            }

            if (1..=methods.len()).contains(&number) {
                return Ok(Some(number - 1));
            }
        }

        eprintln!(
            "Please enter a number between 0 and {}.",
            methods.len()
        );
    }
}

fn choose_run_count() -> io::Result<usize> {
    println!(
        "\nA single construction is enough for a quick test. Use more runs to search for a better solution or to exercise randomized methods."
    );

    loop {
        let input = prompt_line(&format!(
            "Number of constructions per selected method [{DEFAULT_RUNS}]: "
        ))?;

        if input.is_empty() {
            return Ok(DEFAULT_RUNS);
        }

        if let Ok(number) = input.parse::<usize>() {
            if number > 0 {
                return Ok(number);
            }
        }

        eprintln!("Please enter a positive integer.");
    }
}

fn solution_is_better(
    candidate_score: u32,
    candidate_cost: f32,
    candidate_route: &[u8],
    current_score: u32,
    current_cost: f32,
    current_route: &[u8],
) -> bool {
    match candidate_score.cmp(&current_score) {
        Ordering::Greater => true,
        Ordering::Less => false,
        Ordering::Equal => match candidate_cost.total_cmp(&current_cost) {
            Ordering::Less => true,
            Ordering::Greater => false,
            Ordering::Equal => candidate_route < current_route,
        },
    }
}

fn run_method(
    problem: &Problem,
    method: &ExperimentSpec,
    runs: usize,
) -> MethodResult {
    let started_at = Instant::now();
    let mut unique_routes = HashSet::new();
    let mut best: Option<(Vec<u8>, u32, f32, usize)> = None;

    for run_number in 1..=runs {
        let route = heuristic::run_single(problem, &method.selector);
        let status = problem.eval_route(&route);

        unique_routes.insert(route.clone());

        let should_replace = best.as_ref().is_none_or(
            |(best_route, best_score, best_cost, _)| {
                solution_is_better(
                    status.total_score,
                    status.total_consume,
                    &route,
                    *best_score,
                    *best_cost,
                    best_route,
                )
            },
        );

        if should_replace {
            best = Some((
                route,
                status.total_score,
                status.total_consume,
                run_number,
            ));
        }
    }

    let (best_route, best_score, best_cost, best_found_at) =
        best.expect("at least one construction must be executed");

    MethodResult {
        method: method.name.clone(),
        runs,
        unique_routes: unique_routes.len(),
        best_route,
        best_score,
        best_cost,
        best_found_at,
        elapsed: started_at.elapsed(),
    }
}

fn complete_route_string(route: &[u8]) -> String {
    std::iter::once(START_NODE)
        .chain(route.iter().copied())
        .chain(std::iter::once(END_NODE))
        .map(|node| node.to_string())
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn result_is_better(candidate: &MethodResult, current: &MethodResult) -> bool {
    solution_is_better(
        candidate.best_score,
        candidate.best_cost,
        &candidate.best_route,
        current.best_score,
        current.best_cost,
        &current.best_route,
    )
}

fn best_result_index(results: &[MethodResult]) -> usize {
    let mut best_index = 0;

    for index in 1..results.len() {
        if result_is_better(&results[index], &results[best_index]) {
            best_index = index;
        }
    }

    best_index
}

fn build_report(
    instance_path: &Path,
    problem: &Problem,
    results: &[MethodResult],
) -> String {
    let mut report = String::new();

    writeln!(&mut report, "ORIENTEERING PROBLEM EXPERIMENT REPORT").unwrap();
    writeln!(&mut report, "=====================================").unwrap();
    writeln!(&mut report, "Instance: {}", instance_path.display()).unwrap();
    writeln!(&mut report, "Vertices: {}", problem.len).unwrap();
    writeln!(&mut report, "Travel budget: {:.6}", problem.tmax).unwrap();
    writeln!(&mut report).unwrap();

    for result in results {
        writeln!(&mut report, "Method: {}", result.method).unwrap();
        writeln!(&mut report, "  Constructions: {}", result.runs).unwrap();
        writeln!(
            &mut report,
            "  Distinct routes observed: {}",
            result.unique_routes
        )
        .unwrap();
        writeln!(
            &mut report,
            "  Best route first found at construction: {}",
            result.best_found_at
        )
        .unwrap();
        writeln!(&mut report, "  Best score: {}", result.best_score).unwrap();
        writeln!(&mut report, "  Best cost: {:.6}", result.best_cost).unwrap();
        writeln!(
            &mut report,
            "  Feasible: {}",
            if result.best_cost <= problem.tmax {
                "yes"
            } else {
                "no"
            }
        )
        .unwrap();
        writeln!(
            &mut report,
            "  Best route: {}",
            complete_route_string(&result.best_route)
        )
        .unwrap();
        writeln!(
            &mut report,
            "  Elapsed time: {:.6} s",
            result.elapsed.as_secs_f64()
        )
        .unwrap();
        writeln!(&mut report).unwrap();
    }

    let best_index = best_result_index(results);
    let best = &results[best_index];

    writeln!(&mut report, "BEST SOLUTION AMONG THE SELECTED METHODS").unwrap();
    writeln!(&mut report, "Method: {}", best.method).unwrap();
    writeln!(&mut report, "Score: {}", best.best_score).unwrap();
    writeln!(&mut report, "Cost: {:.6}", best.best_cost).unwrap();
    writeln!(
        &mut report,
        "Route: {}",
        complete_route_string(&best.best_route)
    )
    .unwrap();

    report
}

fn sanitize_filename(text: &str) -> String {
    text.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric()
                || character == '-'
                || character == '_'
            {
                character
            } else {
                '_'
            }
        })
        .collect()
}

fn save_report(
    instance_path: &Path,
    results: &[MethodResult],
    report: &str,
) -> io::Result<PathBuf> {
    fs::create_dir_all(REPORTS_DIR)?;

    let instance_name = instance_path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("instance");

    let method_name = if results.len() == 1 {
        results[0].method.as_str()
    } else {
        "all_methods"
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let filename = format!(
        "{}_{}_{}.txt",
        sanitize_filename(instance_name),
        sanitize_filename(method_name),
        timestamp,
    );

    let path = Path::new(REPORTS_DIR).join(filename);
    fs::write(&path, report)?;
    Ok(path)
}

fn update_renderer_map(
    map: &mut Map,
    instance_name: &str,
    result: &MethodResult,
    current_index: usize,
    result_count: usize,
) {
    map.route.clone_from(&result.best_route);
    map.used_cost = result.best_cost;
    map.route_score = result.best_score;
    map.title = format!(
        "{} | {} | {}/{}",
        instance_name,
        result.method,
        current_index + 1,
        result_count,
    );
}

async fn display_results(
    instance_path: &Path,
    points: Vec<Point>,
    problem: &Problem,
    results: &[MethodResult],
) {
    let instance_name = instance_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("instance");

    let mut current_index = best_result_index(results);
    let current = &results[current_index];

    let map = Map {
        route: current.best_route.clone(),
        points,
        tmax: problem.tmax,
        used_cost: current.best_cost,
        title: String::new(),
        route_score: current.best_score,
    };

    let mut renderer = Renderer::new(map);
    update_renderer_map(
        &mut renderer.map,
        instance_name,
        current,
        current_index,
        results.len(),
    );

    loop {
        let mut changed = false;

        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Down) {
            current_index = (current_index + 1) % results.len();
            changed = true;
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::Up) {
            current_index = (current_index + results.len() - 1) % results.len();
            changed = true;
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        if changed {
            update_renderer_map(
                &mut renderer.map,
                instance_name,
                &results[current_index],
                current_index,
                results.len(),
            );
        }

        clear_background(BLACK);
        renderer.draw();

        draw_text(
            "Arrow keys: change method    Esc/Q: close",
            20.0,
            screen_height() - 15.0,
            18.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}

async fn run_program() -> Result<(), Box<dyn Error>> {
    let data_directory = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_DATA_DIR));

    let instance_files = discover_instance_files(&data_directory)?;
    let instance_path = choose_instance(&data_directory, &instance_files)?;

    let methods = available_methods();
    let method_choice = choose_method(&methods)?;
    let runs = choose_run_count()?;

    let selected_methods = match method_choice {
        Some(index) => vec![methods[index].clone()],
        None => methods,
    };

    let raw_input = fs::read_to_string(&instance_path)?;
    let problem = Problem::new(&raw_input);
    let points = Point::parse_all(&raw_input);

    if points.len() != problem.len {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the problem parser and renderer parsed different numbers of vertices",
        )
        .into());
    }

    println!(
        "\nRunning {} selected method(s), {} construction(s) per method...\n",
        selected_methods.len(),
        runs,
    );

    let mut results = Vec::with_capacity(selected_methods.len());

    for method in &selected_methods {
        println!("Running {}...", method.name);
        results.push(run_method(&problem, method, runs));
    }

    let report = build_report(&instance_path, &problem, &results);
    let report_path = save_report(&instance_path, &results, &report)?;

    println!("\n{report}");
    println!("Report saved to {}", report_path.display());
    println!("Opening the best-route viewer...");

    display_results(&instance_path, points, &problem, &results).await;
    Ok(())
}

#[macroquad::main(window_conf)]
async fn main() {
    if let Err(error) = run_program().await {
        eprintln!("\nError: {error}");
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Orienteering Problem - Best Route Viewer".to_owned(),
        window_width: 1100,
        window_height: 750,
        window_resizable: true,
        ..Default::default()
    }
}

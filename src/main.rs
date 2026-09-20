use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use macroquad::prelude::Conf;

mod problem;
mod rendering;

use problem::heuristic;
use problem::heuristic::ExperimentSpec;
use problem::heuristic::strategy::{
    marginal_envy,
    marginal_smart,
    total_envy,
    total_smart,
    Selector,
};
use problem::Problem;


// =============================================================================
// Configuration
// =============================================================================

/// Number of structurally distinct routes retained for each method--instance
/// pair.
const N_ROUTES: usize = 6;

/// Avoid an infinite loop when a method generates very few distinct routes.
const MAX_ATTEMPTS: usize = 10_000;

const DATA_DIR: &str = "data";
const OUTPUT_FILE: &str = "routes_visual_comparison.csv";

const START_NODE: u8 = 0;
const END_NODE: u8 = 1;

const SELECTED_PROBLEMS: [&str; 3] = [
    "set_64_1_45.txt",
    "set_66_1_060.txt",
    "tsiligirides_problem_2_budget_35.txt",
];


// =============================================================================
// Methods shown in the final comparison of the report
// =============================================================================

const RANDOM_METHOD_NAMES: [&str; 4] = [
    "selected_random",
    "selected_weighted_random",
    "selected_random_random_tie",
    "random_feasible_insertion",
];


/// Build exactly the eight methods used in the report's final comparison.
///
/// `heuristic::experiment_specs()` is deliberately a randomized-only battery,
/// so the four deterministic reference methods must be created explicitly.
fn comparison_methods() -> Vec<ExperimentSpec> {
    let mut selected = vec![
        ExperimentSpec {
            name: "marginal_smart".into(),
            selector: Selector::By(marginal_smart),
        },
        ExperimentSpec {
            name: "total_smart".into(),
            selector: Selector::By(total_smart),
        },
        ExperimentSpec {
            name: "marginal_envy".into(),
            selector: Selector::By(marginal_envy),
        },
        ExperimentSpec {
            name: "total_envy".into(),
            selector: Selector::By(total_envy),
        },
    ];

    let randomized = heuristic::experiment_specs();

    for wanted_name in RANDOM_METHOD_NAMES {
        let spec = randomized
            .iter()
            .find(|spec| spec.name == wanted_name)
            .unwrap_or_else(|| {
                let available = randomized
                    .iter()
                    .map(|spec| spec.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");

                panic!(
                    "Required randomized method {wanted_name:?} was not found. \
                     Available methods: [{available}]"
                )
            });

        selected.push(spec.clone());
    }

    selected
}


// =============================================================================
// Recursive problem-file discovery
// =============================================================================

fn find_file(dir: &Path, target: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(found) = find_file(&path, target) {
                return Some(found);
            }

            continue;
        }

        if entry.file_name().to_string_lossy() == target {
            return Some(path);
        }
    }

    None
}


// =============================================================================
// Route formatting
//
// The heuristic stores only optional vertices. The CSV consumed by Colab
// receives the complete path 0 -> ... -> 1.
// =============================================================================

fn complete_route_string(route: &[u8]) -> String {
    let mut nodes = Vec::with_capacity(route.len() + 2);

    nodes.push(START_NODE.to_string());
    nodes.extend(route.iter().map(u8::to_string));
    nodes.push(END_NODE.to_string());

    nodes.join(" ")
}


// =============================================================================
// Main
// =============================================================================

fn main() {
    let methods = comparison_methods();

    assert!(
        !methods.is_empty(),
        "No comparison methods were configured."
    );

    eprintln!("Methods selected for visual comparison:");

    for spec in &methods {
        eprintln!("  - {}", spec.name);
    }

    eprintln!();

    let header =
        "problem,method,sample,score,cost,attempt_found,route";

    let mut csv = String::new();
    csv.push_str(header);
    csv.push('\n');

    // stdout remains valid CSV. Diagnostics go only to stderr.
    println!("{header}");

    for problem_name in SELECTED_PROBLEMS {
        let path = find_file(Path::new(DATA_DIR), problem_name)
            .unwrap_or_else(|| {
                panic!(
                    "Could not find {problem_name:?} recursively inside \
                     {DATA_DIR:?}."
                )
            });

        let raw_input = fs::read_to_string(&path)
            .unwrap_or_else(|err| {
                panic!(
                    "Could not read {}: {err}",
                    path.display(),
                )
            });

        let problem = Problem::new(&raw_input);

        for spec in &methods {
            let mut unique_routes: HashSet<Vec<u8>> = HashSet::new();
            let mut selected_routes: Vec<(Vec<u8>, usize)> =
                Vec::with_capacity(N_ROUTES);
            let mut attempts = 0usize;

            while selected_routes.len() < N_ROUTES
                && attempts < MAX_ATTEMPTS
            {
                attempts += 1;

                let route = heuristic::run_single(
                    &problem,
                    &spec.selector,
                );

                if !unique_routes.insert(route.clone()) {
                    continue;
                }

                selected_routes.push((route, attempts));
            }

            for (sample_index, (route, attempt_found)) in
                selected_routes.iter().enumerate()
            {
                let status = problem.eval_route(route);
                let route_text = complete_route_string(route);

                let row = format!(
                    "{},{},{},{},{},{},\"{}\"",
                    problem_name,
                    spec.name,
                    sample_index + 1,
                    status.total_score,
                    status.total_consume,
                    attempt_found,
                    route_text,
                );

                println!("{row}");
                csv.push_str(&row);
                csv.push('\n');
            }

            eprintln!(
                "{} / {}: {} distinct routes found in {} attempts",
                problem_name,
                spec.name,
                selected_routes.len(),
                attempts,
            );

            if selected_routes.len() < N_ROUTES {
                eprintln!(
                    "  warning: the method did not generate six distinct \
                     routes before MAX_ATTEMPTS={MAX_ATTEMPTS}."
                );
            }
        }
    }

    fs::write(OUTPUT_FILE, &csv)
        .unwrap_or_else(|err| {
            panic!("Could not write {OUTPUT_FILE:?}: {err}")
        });

    eprintln!();
    eprintln!("Visual-comparison CSV saved to {OUTPUT_FILE:?}.");
    eprintln!(
        "Expected maximum rows: {} problems x {} methods x {} routes = {}.",
        SELECTED_PROBLEMS.len(),
        methods.len(),
        N_ROUTES,
        SELECTED_PROBLEMS.len() * methods.len() * N_ROUTES,
    );
}


// Kept for projects in which the rendering module still reads this function.
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Visualizador de Caminho".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

use std::collections::{
    hash_map::Entry,
    HashMap,
    HashSet,
};
use std::fs;
use std::path::{Path, PathBuf};

use macroquad::prelude::Conf;

mod problem;
mod rendering;

use problem::heuristic;
use problem::heuristic::strategy::{
    marginal_envy,
    marginal_smart,
    total_envy,
    total_smart,
    Selector,
};
use problem::heuristic::ExperimentSpec;
use problem::Problem;


// =============================================================================
// Configuration
// =============================================================================

/// Every method--problem pair is executed exactly this many times.
const N_RUNS: usize = 10_000;

/// Number of routes retained after diversity maximization.
const N_ROUTES: usize = 6;

/// Exact-diameter search is quadratic. Above this number of unique routes, a
/// deterministic two-sweep approximation supplies the initial pair.
const EXACT_DIAMETER_LIMIT: usize = 2_000;

/// One-swap local improvement passes for the max-min diversity objective.
const MAX_LOCAL_SEARCH_PASSES: usize = 8;

const DATA_DIR: &str = "data";
const OUTPUT_FILE: &str = "routes_visual_comparison.csv";

const START_NODE: u8 = 0;
const END_NODE: u8 = 1;

/// Three distinct benchmark families selected in the previous analysis.
const SELECTED_PROBLEMS: [&str; 3] = [
    "set_64_1_35.txt",
    "set_66_1_035.txt",
    "tsiligirides_problem_2_budget_35.txt",
];

/// These are still emitted for the final visual comparison. They do not affect
/// instance selection.
const RANDOM_VISUAL_METHODS: [&str; 4] = [
    "selected_random",
    "selected_weighted_random",
    "selected_random_random_tie",
    "random_feasible_insertion",
];


// =============================================================================
// Methods shown in the final comparison
// =============================================================================

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

    for wanted_name in RANDOM_VISUAL_METHODS {
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
                    "Required method {wanted_name:?} was not found. \
                     Available randomized methods: [{available}]"
                )
            });

        selected.push(spec.clone());
    }

    selected
}


// =============================================================================
// Problem-file discovery
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
// Route representation and structural distance
// =============================================================================

#[derive(Clone, Copy)]
struct RouteOccurrence {
    first_attempt: usize,
    frequency: usize,
}


struct RouteRecord {
    route: Vec<u8>,
    arcs: HashSet<(u8, u8)>,
    first_attempt: usize,
    frequency: usize,
}


fn route_arcs(route: &[u8]) -> HashSet<(u8, u8)> {
    let mut arcs = HashSet::with_capacity(route.len() + 1);
    let mut previous = START_NODE;

    for &node in route {
        arcs.insert((previous, node));
        previous = node;
    }

    arcs.insert((previous, END_NODE));
    arcs
}


fn jaccard_distance(
    left: &HashSet<(u8, u8)>,
    right: &HashSet<(u8, u8)>,
) -> f64 {
    let intersection = left.intersection(right).count();
    let union = left.len() + right.len() - intersection;

    if union == 0 {
        0.0
    } else {
        1.0 - intersection as f64 / union as f64
    }
}


fn distance(records: &[RouteRecord], left: usize, right: usize) -> f64 {
    jaccard_distance(&records[left].arcs, &records[right].arcs)
}


fn collect_routes(
    problem: &Problem,
    selector: &Selector,
) -> Vec<RouteRecord> {
    let mut observed: HashMap<Vec<u8>, RouteOccurrence> = HashMap::new();

    for attempt in 1..=N_RUNS {
        let route = heuristic::run_single(problem, selector);

        match observed.entry(route) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().frequency += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(RouteOccurrence {
                    first_attempt: attempt,
                    frequency: 1,
                });
            }
        }
    }

    let mut records: Vec<RouteRecord> = observed
        .into_iter()
        .map(|(route, occurrence)| RouteRecord {
            arcs: route_arcs(&route),
            route,
            first_attempt: occurrence.first_attempt,
            frequency: occurrence.frequency,
        })
        .collect();

    // HashMap iteration is deliberately ignored. Lexicographic ordering makes
    // every exact-distance tie deterministic for a fixed set of routes.
    records.sort_by(|left, right| left.route.cmp(&right.route));

    debug_assert_eq!(
        records.iter().map(|record| record.frequency).sum::<usize>(),
        N_RUNS,
    );

    records
}


// =============================================================================
// Six-route maximum-diversity heuristic
// =============================================================================

fn exact_farthest_pair(records: &[RouteRecord]) -> (usize, usize) {
    let mut best_pair = (0usize, 1usize);
    let mut best_distance = distance(records, 0, 1);

    for left in 0..records.len() - 1 {
        for right in left + 1..records.len() {
            let candidate_distance = distance(records, left, right);

            if candidate_distance > best_distance {
                best_distance = candidate_distance;
                best_pair = (left, right);
            }
        }
    }

    best_pair
}


fn farthest_from(
    records: &[RouteRecord],
    anchor: usize,
) -> usize {
    let mut best_index = if anchor == 0 { 1 } else { 0 };
    let mut best_distance = f64::NEG_INFINITY;

    for candidate in 0..records.len() {
        if candidate == anchor {
            continue;
        }

        let candidate_distance = distance(records, anchor, candidate);

        if candidate_distance > best_distance {
            best_distance = candidate_distance;
            best_index = candidate;
        }
    }

    best_index
}


fn approximate_farthest_pair(records: &[RouteRecord]) -> (usize, usize) {
    let first = farthest_from(records, 0);
    let second = farthest_from(records, first);
    (first, second)
}


fn subset_distance_sum(records: &[RouteRecord], selected: &[usize]) -> f64 {
    let mut total = 0.0;

    for left_position in 0..selected.len() {
        for right_position in left_position + 1..selected.len() {
            total += distance(
                records,
                selected[left_position],
                selected[right_position],
            );
        }
    }

    total
}


fn minimum_pairwise_distance(
    records: &[RouteRecord],
    selected: &[usize],
) -> f64 {
    if selected.len() < 2 {
        return 0.0;
    }

    let mut minimum = f64::INFINITY;

    for left_position in 0..selected.len() - 1 {
        for right_position in left_position + 1..selected.len() {
            minimum = minimum.min(distance(
                records,
                selected[left_position],
                selected[right_position],
            ));
        }
    }

    minimum
}


fn mean_pairwise_distance(
    records: &[RouteRecord],
    selected: &[usize],
) -> f64 {
    if selected.len() < 2 {
        return 0.0;
    }

    let pairs = selected.len() * (selected.len() - 1) / 2;
    subset_distance_sum(records, selected) / pairs as f64
}


fn greedily_complete_subset(
    records: &[RouteRecord],
    selected: &mut Vec<usize>,
    target_size: usize,
) {
    const EPSILON: f64 = 1e-12;

    while selected.len() < target_size {
        let selected_set: HashSet<usize> = selected.iter().copied().collect();
        let mut best_candidate = None;
        let mut best_sum = f64::NEG_INFINITY;
        let mut best_minimum = f64::NEG_INFINITY;

        for candidate in 0..records.len() {
            if selected_set.contains(&candidate) {
                continue;
            }

            let mut distance_sum = 0.0;
            let mut minimum_distance = f64::INFINITY;

            for &chosen in selected.iter() {
                let value = distance(records, candidate, chosen);
                distance_sum += value;
                minimum_distance = minimum_distance.min(value);
            }

            if minimum_distance > best_minimum + EPSILON
                || ((minimum_distance - best_minimum).abs() <= EPSILON
                    && distance_sum > best_sum + EPSILON)
            {
                best_candidate = Some(candidate);
                best_sum = distance_sum;
                best_minimum = minimum_distance;
            }
        }

        selected.push(
            best_candidate.expect("no candidate available while completing subset")
        );
    }
}


fn improve_by_one_swaps(
    records: &[RouteRecord],
    selected: &mut [usize],
) {
    const EPSILON: f64 = 1e-12;

    for _ in 0..MAX_LOCAL_SEARCH_PASSES {
        let selected_set: HashSet<usize> = selected.iter().copied().collect();
        let current_minimum = minimum_pairwise_distance(records, selected);
        let current_sum = subset_distance_sum(records, selected);
        let mut best_minimum = current_minimum;
        let mut best_sum = current_sum;
        let mut best_swap = None;

        for position in 0..selected.len() {
            for candidate in 0..records.len() {
                if selected_set.contains(&candidate) {
                    continue;
                }

                let mut trial = selected.to_vec();
                trial[position] = candidate;
                let trial_minimum =
                    minimum_pairwise_distance(records, &trial);
                let trial_sum = subset_distance_sum(records, &trial);

                if trial_minimum > best_minimum + EPSILON
                    || ((trial_minimum - best_minimum).abs() <= EPSILON
                        && trial_sum > best_sum + EPSILON)
                {
                    best_minimum = trial_minimum;
                    best_sum = trial_sum;
                    best_swap = Some((position, candidate));
                }
            }
        }

        let Some((position, candidate)) = best_swap else {
            break;
        };

        selected[position] = candidate;
    }
}


fn select_diverse_routes(
    records: &[RouteRecord],
    wanted: usize,
) -> Vec<usize> {
    let target_size = wanted.min(records.len());

    match target_size {
        0 => return Vec::new(),
        1 => return vec![0],
        _ => {}
    }

    if records.len() <= target_size {
        return (0..records.len()).collect();
    }

    let initial_pair = if records.len() <= EXACT_DIAMETER_LIMIT {
        exact_farthest_pair(records)
    } else {
        approximate_farthest_pair(records)
    };

    let mut selected = vec![initial_pair.0, initial_pair.1];
    greedily_complete_subset(records, &mut selected, target_size);
    improve_by_one_swaps(records, &mut selected);
    selected
}


fn route_distances_to_selection(
    records: &[RouteRecord],
    selected: &[usize],
    target_position: usize,
) -> (f64, f64) {
    if selected.len() < 2 {
        return (0.0, 0.0);
    }

    let target = selected[target_position];
    let mut sum = 0.0;
    let mut minimum = f64::INFINITY;

    for (other_position, &other) in selected.iter().enumerate() {
        if other_position == target_position {
            continue;
        }

        let value = distance(records, target, other);
        sum += value;
        minimum = minimum.min(value);
    }

    (sum / (selected.len() - 1) as f64, minimum)
}


// =============================================================================
// CSV formatting
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

    eprintln!("Selected problems:");
    for problem_name in SELECTED_PROBLEMS {
        eprintln!("  - {problem_name}");
    }

    eprintln!("\nVisual-comparison methods:");
    for spec in &methods {
        eprintln!("  - {}", spec.name);
    }

    let header = concat!(
        "problem,method,sample,score,cost,attempt_found,frequency,",
        "mean_distance_to_selected,",
        "min_distance_to_selected,route"
    );
    let mut output = String::new();
    output.push_str(header);
    output.push('\n');
    println!("{header}");

    for problem_name in SELECTED_PROBLEMS {
        let path = find_file(
            Path::new(DATA_DIR),
            problem_name,
        )
        .unwrap_or_else(|| {
            panic!(
                "Could not find {:?} recursively inside {:?}.",
                problem_name,
                DATA_DIR,
            )
        });

        let raw_input = fs::read_to_string(&path)
            .unwrap_or_else(|err| {
                panic!("Could not read {}: {err}", path.display())
            });
        let problem = Problem::new(&raw_input);

        for spec in &methods {
            eprintln!(
                "\n{} / {}: running exactly {} constructions...",
                problem_name,
                spec.name,
                N_RUNS,
            );

            let records = collect_routes(&problem, &spec.selector);

            if records.len() < N_ROUTES {
                eprintln!(
                    "WARNING: {} / {} produced only {} unique route(s) in \
                     {} constructions; it is impossible to emit six distinct \
                     routes for this pair.",
                    problem_name,
                    spec.name,
                    records.len(),
                    N_RUNS,
                );
            }

            let selected = select_diverse_routes(&records, N_ROUTES);
            let selected_diversity =
                mean_pairwise_distance(&records, &selected);
            let selected_minimum =
                minimum_pairwise_distance(&records, &selected);

            eprintln!(
                "{} / {}: {} unique routes; selected {} with mean pairwise \
                 arc-Jaccard distance {:.6} and minimum {:.6}.",
                problem_name,
                spec.name,
                records.len(),
                selected.len(),
                selected_diversity,
                selected_minimum,
            );

            for (sample_index, &record_index) in selected.iter().enumerate() {
                let record = &records[record_index];
                let status = problem.eval_route(&record.route);
                let route_text = complete_route_string(&record.route);
                let (mean_distance, min_distance) =
                    route_distances_to_selection(
                        &records,
                        &selected,
                        sample_index,
                    );

                let row = format!(
                    "{},{},{},{},{},{},{},{},{},\"{}\"",
                    problem_name,
                    spec.name,
                    sample_index + 1,
                    status.total_score,
                    status.total_consume,
                    record.first_attempt,
                    record.frequency,
                    mean_distance,
                    min_distance,
                    route_text,
                );

                println!("{row}");
                output.push_str(&row);
                output.push('\n');
            }
        }
    }

    fs::write(OUTPUT_FILE, output)
        .unwrap_or_else(|err| {
            panic!("Could not write {OUTPUT_FILE:?}: {err}")
        });

    eprintln!("\nCompleted visual-route extraction.");
    eprintln!("Routes:  {OUTPUT_FILE}");
    eprintln!(
        "Each method--problem pair used exactly {N_RUNS} constructions."
    );
}


// Kept because it is still used by the rendering module.
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Visualizador de Caminho".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

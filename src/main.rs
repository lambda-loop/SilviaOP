use std::fs;

mod problem;
mod rendering;

use problem::Problem;
use problem::heuristic;
use problem::heuristic::*;

use problem::metaheuristics::simulated_annealing as sa;
use rendering::sa::Voyeur;
use rendering::window_conf;
use rendering::Point;

#[macroquad::main(window_conf)]
async fn main() {
    let raw_input =
        // fs::read_to_string("data/set_64_1_60.txt").unwrap();
        // fs::read_to_string("data/tsiligirides_problem_3_budget_045.txt").unwrap();
        fs::read_to_string("data/tsiligirides_problem_2_budget_32.txt").unwrap();

    let problem = Problem::new(&raw_input);

    let initial = simple_heuristic(&problem, greedy);
    let initial = since_you_already_best(&problem, initial);

    let unvisited = (2..problem.len as u8).collect();

    let points = Point::problem(&raw_input);

    let mut state = sa::State::new(
        problem,
        initial.clone(),
        unvisited,
        points,
    );

    state.run(&raw_input, initial).await;
}

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
    marginal_lazy,
    marginal_envy,
    marginal_smart,
    total_envy,
    total_smart,
    Selector,
};
use problem::heuristic::ExperimentSpec;
use problem::Problem;

use problem::metaheuristics::grasp;

const FILE_NAME: &'static str = "set_64_1_65.txt";
pub fn main() {
    // let builder = grasp::build::build_classical;
    let builder = grasp::build::build_rand;
    let input = fs::read_to_string(format!("data/{}", FILE_NAME)).unwrap();
    let p = Problem::new(&input);
    let mut rng = rand::rng();
    let best_tour = grasp::grasp_classical(&p, 20, 0.2, &mut rng, true, builder);
    let status = p.eval_route(&best_tour);
    println!("tour: {:?}", &best_tour);
    println!("score: {}", status.total_score);
    println!("consume: {}", status.total_consume);
    unsafe {
        let evals = problem::EVALS;
        println!("EVALS: {}", evals);
    };
    // println!("{}", &input);
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

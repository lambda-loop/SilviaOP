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

use problem::metaheuristics::grasp;

const FILE_NAME: &'static str = "set_64_1_65.txt";
pub fn main() {
    let input = fs::read_to_string(format!("data/{}", FILE_NAME)).unwrap();
    let p = Problem::new(&input);
    let mut rng = rand::rng();
    grasp::grasp_classical(&p, 0.2, &mut rng);
    println!("{}", &input);
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

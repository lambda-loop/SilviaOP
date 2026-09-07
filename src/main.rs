use std::fs;

mod problem;
mod rendering;

use problem::Problem;
use problem::heuristic;
use problem::heuristic::*;

// use problem::metaheuristics::simulated_annealing as sa;
// use rendering::sa::Voyeur;
// use rendering::window_conf;


use rendering::renderer::*;
use rendering::point::*;
use macroquad::prelude::Conf;
use macroquad::window::*;

#[macroquad::main(window_conf)]
async fn main() {
    let raw_input = fs::read_to_string("data/set_64_1_15.txt").unwrap();
    let problem = Problem::new(&raw_input);
    let points = Point::parse_all(&raw_input);

    let map = Map {
        route: vec![],
        points, tmax: problem.tmax,
        used_cost: 0.,
        title: "".to_string(),
        route_score: 0,
    };

    let renderer = Renderer::new(map);
    loop {
        renderer.draw();
        next_frame().await;
    }
    
    
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Visualizador de Caminho".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

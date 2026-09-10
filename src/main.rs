use std::fs;

mod problem;
mod rendering;

use problem::heuristic;
use problem::heuristic::*;
use problem::Problem;

// use problem::metaheuristics::simulated_annealing as sa;
// use rendering::sa::Voyeur;
// use rendering::window_conf;

use macroquad::prelude::{is_key_pressed, Conf, KeyCode};
use macroquad::window::*;
use rendering::point::*;
use rendering::renderer::*;

use problem::heuristic::run_multi_experiment;
use problem::heuristic::strategy::greedy;
use problem::heuristic::strategy::smart;

// #[macroquad::main(window_conf)]
// async fn main() {

fn main() {
    let dir = fs::read_dir("data").unwrap();
    for file in dir {
        let file = file.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        };

        let problem_name = file.file_name().clone().into_string().unwrap();
        if !problem_name.ends_with(".txt") {
            continue;
        }

        let raw_input = fs::read_to_string(file.path()).unwrap();
        let problem = Problem::new(&raw_input);

        println!("{}", problem_name);

        let ers = one_experiment(&problem, &problem_name);

        // println!("{:?}", &ers);
    }

    let raw_input = fs::read_to_string("data/set_64_1_65.txt").unwrap();
    let problem = Problem::new(&raw_input);
    let points = Point::parse_all(&raw_input);

    let ers = run_single_experiment(&problem, "64".to_string(), greedy, "greedy".to_string());
    println!("{}", ers.to_csv());
    let r = run_single(&problem, greedy);
    let sts = problem.eval_route(&r);
    println!("{}", r.len());

    let map = Map {
        route: r,
        points,
        tmax: problem.tmax,
        used_cost: sts.total_consume,
        title: "".to_string(),
        route_score: sts.total_score,
    };

    // let renderer = Renderer::new(map);
    // loop {
    //     if is_key_pressed(KeyCode::Q) {
    //         break;
    //     }

    //     renderer.draw();
    //     next_frame().await;
    // }
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

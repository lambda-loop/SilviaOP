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

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crossbeam_channel::unbounded as uchan;

fn main() {
    let dir = fs::read_dir("data").unwrap();
    // let mut all_ers = Vec::new();

    let (tp, rp) = uchan();
    let (tr, rr) = mpsc::channel();
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

        tp.send((problem_name, problem));
        // println!("{}", problem_name);

        // let ers: Vec<_> = multi_results(&problem, &problem_name)
        //     .iter()
        //     .map(|er| er.to_csv())
        //     .collect();
    }
    drop(tp);

    let mut handles = Vec::new();

    for _ in 0..12 {
        let rp = rp.clone();
        let tr = tr.clone();

        let handle = thread::spawn(move || {
            while let Ok((problem_name, problem)) = rp.recv() {
                let ers: Vec<_> = multi_results(&problem, &problem_name)
                    .iter()
                    .map(|er| er.to_csv())
                    .collect();

                tr.send(ers).unwrap();
            }
        });

        handles.push(handle);
    }

    drop(rp);
    drop(tr);

    for handle in handles {
        handle.join().unwrap();
    }

    let header = heuristic::EResult::header();
    let mut content = Vec::new();

    content.push(format!("{header}\n"));

    while let Ok(lines) = rr.recv() {
        for line in lines {
            content.push(format!("{line}\n"));
        }
    }

    fs::write("out2.csv", content.concat()).unwrap();
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

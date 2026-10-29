use std::collections::{
    hash_map::Entry,
    HashMap,
    HashSet,
};
use std::fs;
use std::path::{Path, PathBuf};

use std::sync::{
    mpsc::{self, Receiver, SyncSender}
};
use std::thread;
use macroquad::prelude::Conf;

mod config;
mod solver;
mod problem;
mod rendering;
mod experiment;

use rand::*;

use problem::{
    Problem, Tour,
};
use rand::rngs;
use rendering::renderer::*;
use experiment::*;

use solver::metaheuristics::tsp::{
    ils, naive_greedy,
};
#[macroquad::main(window_conf)]
async fn main() {
    let (sx, rx) = mpsc::sync_channel(10);

    print!("{}", Conclusion::header());

    thread::spawn(move || oplib_problems(sx));

    for _ in 0..120 {
        rx.recv();
    }

    let (content, name) = rx
        .recv()
        .expect("Nenhum arquivo .oplib recebido");

    let problem = Problem::from_op(&content);
    let mut tour = Tour::new(problem.len);

    let mut t_rng = rand::rng();
    for i in 0..problem.len {
        if t_rng.random_bool(0.1) {
            tour.push(i as u16);
        }
    }



    let mut map = Map::from_op(
        &content,
        tour,
        0.0,
        name,
        0,
    );
    let mut t_rng = rand::rng();
    let mut rng = rngs::SmallRng::from_rng(&mut t_rng);

    let mut renderer = Renderer::new(map);

    loop {
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if is_key_pressed(KeyCode::I) {
            naive_greedy::heuristic(renderer.map.tour.as_mut_slice(), &problem.costs);
            
            ils::ils(&mut renderer.map.tour, &mut rng, &problem.costs, 100_000);
            // renderer.map.tour
            
        }

        clear_background(BLACK);
        renderer.draw();

        next_frame().await;
    }
}

fn oplib_problems(sx: SyncSender<(String, String)>) {
    let dir = fs::read_dir("OPLib").unwrap(); // let mut inputs = Vec::new();
    let count = dir.count();
    let dir = fs::read_dir("OPLib").unwrap(); 
    println!("dir count: {}", count);
    for file in dir {
        let entry = file.unwrap();
        match entry.file_type() {
            Ok(ft) if ft.is_file() => {
                let file_name = entry.file_name();
                let file_name = file_name.to_string_lossy();
                if file_name.ends_with(".oplib") {
                    let content = fs::read_to_string(entry.path()).unwrap();
                    sx.send((
                        content,
                        // Problem::from_op(&content),
                        String::from(file_name),
                    ));
                    // inputs.push((
                    //     Problem::from_op(&content),
                    //     file_name.to_string(),
                    // ))
                }
            },
            Err(_) => panic!(),
            _ => continue,
        }
        
    }
}

pub fn all_problems() -> Vec<(Problem, String)> {
    let dir = fs::read_dir("data").unwrap();
    let mut inputs = Vec::new();
    for file in dir {
        let entry = file.unwrap();
        match entry.file_type() {
            Ok(ft) if ft.is_file() => {
                let file_name = entry.file_name();
                let file_name = file_name.to_string_lossy();
                if file_name.ends_with(".txt") {
                    let content = fs::read_to_string(entry.path()).unwrap();
                    inputs.push((
                        Problem::from_classical(&content),
                        file_name.to_string(),
                    ))
                }
            },
            Err(_) => panic!(),
            _ => continue,
        }
        
    }

    inputs
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

use macroquad::prelude::{
    clear_background, is_key_pressed, next_frame, KeyCode, BLACK,
};

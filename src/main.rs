use std::collections::{
    hash_map::Entry,
    HashMap,
    HashSet,
};
use std::fs;
use std::path::{Path, PathBuf};

use std::sync::{
    mpsc::{self, Receiver, Sender}
};
use std::thread;
use macroquad::prelude::Conf;

mod config;
mod solver;
mod problem;
mod rendering;
mod experiment;

use problem::{
    Problem, Tour,
};
use rendering::renderer::*;
use experiment::*;

// use problem::metaheuristics::grasp;
pub fn main() {
    // let x = 3;
    // let p0 = &raw const x;

    // const x = 3;
    // const p0 = &x;


    // var x = 3;
    // const p0 = &x;

    let (sx, rx) = mpsc::channel();

    let header = &Conclusion::header();
    print!("{header}");
    // let ps = all_problems();
    // memetic::Memetic::run_all_ms_in_all_ps(ps, 30);
    // println!("acabou-simples!");
    thread::spawn(move || oplib_problems(sx));
    let (content, name) = rx.recv().unwrap();
    let map = Map::from_op(
        &content,
        Tour::new(100),
        0., 
        name, 
        0,
    );

    let renderer = Renderer::new(map);
    renderer.draw();

    // println!("{}", ps[0]);
    // memetic::Memetic::run_all_ms_in_all_ps(ps, 30);
    // println!("acabou-tudo!");

}

fn oplib_problems(sx: Sender<(String, String)>) {
    let dir = fs::read_dir("OPLib").unwrap();
    // let mut inputs = Vec::new();
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

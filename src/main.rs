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
use problem::metaheuristics;
use metaheuristics::*;

use metaheuristics::experiment::Conclusion;

const FILE_NAME: &'static str = "set_64_1_65.txt";
pub fn main() {
    // let builder = grasp::build::build_classical;
    // let builder = grasp::build::build_rand;
    // let input = fs::read_to_string(format!("data/{}", FILE_NAME)).unwrap();
    // let p = Problem::new(&input);
    // let mut rng = rand::rng();
    // let best_tour = grasp::grasp_classical(&p, 20, 0.2, &mut rng, false, builder);
    let mut ps = all_problems();
    // let mut oplib_ps = oplib_problems();
    // ps.append(&mut oplib_ps);
    // let ps = ps.into_iter().filter(|(p, _)| p.len < 100).collect();

    // let cs =
    let header = &Conclusion::header();
    print!("{header}");
    grasp::Grasp::run_all_ms_in_all_ps(ps, 100);
    println!("acabou!")

    // let mut contents = String::new();
    // contents.push_str(&Conclusion::header());

    // for c in cs.into_iter() {
    //     contents.push_str(&c.to_csv());
    // }

    // fs::write("grasps3.csv", &contents);
    // let ms = grasp::Grasp::all_ms();
    // for m in ms.iter() {
    //     println!("{:?}", m.0.alpha);
    // }

    // let (mut grasp, a) = ms[0].clone();
    // let best_tour = grasp.shot(&p).tour;
    // let status = p.eval_route(&best_tour);
    // println!("tour: {:?}", &best_tour);
    // println!("score: {}", status.total_score);
    // println!("consume: {}", status.total_consume);
    // unsafe {
    //     let evals = problem::EVALS;
    //     println!("EVALS: {}", evals);
    // };
    // println!("{}", &input);
}

use problem::oplib;
fn oplib_problems() -> Vec<(Problem, String)> {
    let dir = fs::read_dir("OPLib").unwrap();
    let mut inputs = Vec::new();
    for file in dir {
        let entry = file.unwrap();
        match entry.file_type() {
            Ok(ft) if ft.is_file() => {
                let file_name = entry.file_name();
                let file_name = file_name.to_string_lossy();
                if file_name.ends_with(".oplib") {
                    let content = fs::read_to_string(entry.path()).unwrap();
                    inputs.push((
                        oplib::parse(&content),
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
                        Problem::new(&content),
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

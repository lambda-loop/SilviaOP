
use fixedbitset::FixedBitSet as Set;

// mod experiemnt;
mod strategy;
mod path_relinking;

use path_relinking::path_relink_all;

use strategy::*;
use crate::problem::Problem;

pub mod build;
pub mod local_search;

use build::*;
// use build01::build01;
// use build02::build02;
// use build03::build03;
// use build04::build04;

use local_search::local_search;
use rand::*;
use rand::rngs::ThreadRng;

const NUM_BUILDS: usize = 20;

#[derive(Clone, Debug)]
pub enum Relink{
    Classical,
    Weird, 
    None,
}

#[derive(Clone)]
pub struct Grasp {
    pub num_builds: usize,
    pub alpha: f32,
    pub rng: ThreadRng,
    pub relink: Relink,
    pub builder: Builder,
}

use super::Method;
impl Method for Grasp {
    fn shot(&mut self, p: &Problem) -> ER {
        let start = std::time::Instant::now();
        let mut tours = Vec::new();
        
        for i in 0..self.num_builds {
            // let tour = build_classical_03(p, alpha, rng);
            let tour = (self.builder)(p, self.alpha, &mut self.rng);
            let mut visited = Set::with_capacity(p.len);
            visited.insert(0); visited.insert(1);
            for &v in tour.iter() { visited.insert(v as usize); }
            let tour = local_search(p, tour, &mut visited);
            // println!("{:?}", i);
            tours.push(tour);
        }

        let tour = match self.relink {
            Relink::Classical => {
                path_relink_all(
                    p,
                    tours,
                    true,
                    &mut self.rng,
                )
            }
            
            Relink::Weird => {
                path_relink_all(
                    p,
                    tours,
                    false,
                    &mut self.rng,
                )
            }
            
            Relink::None => {
                let best = tours
                    .into_iter()
                    .max_by(|l, r| {
                        let l = p.eval_route(l).total_score;
                        let r = p.eval_route(r).total_score;
                        l.cmp(&r)
                    }).unwrap();
                best
            },
        };

        ER {
            tour,
            time: start.elapsed(),
        }
    }

    fn all_ms() -> Vec<(Self, String)> {
        let mut ms = Vec::new();
        for x in 1..10 {
            for r in RELINKERS {
                let rng = rand::rng();
                let alpha = x as f32 / 10.;
                let name = format!("grasp-classical-{:?}-{:?}", alpha, r);
                ms.push( (Grasp {
                    num_builds: NUM_BUILDS,
                    alpha,
                    rng,
                    relink: r.clone(),
                    builder: build_classical,
                }, name));

                let rng = rand::rng();
                let alpha = x as f32 / 10.;
                let name = format!("grasp-rand-{:?}-{:?}", alpha, r);
                ms.push( (Grasp {
                    num_builds: NUM_BUILDS,
                    alpha,
                    rng,
                    relink: r,
                    builder: build_rand,
                }, name));
            }
        }


        ms
    }
    
}
// TODO: for nos builders..?

const RELINKERS: [Relink;3] = [
    Relink::Classical,
    Relink::Weird,
    Relink::None,
];

pub fn grasp_classical(
    p: &Problem,
    num_builds: usize,
    alpha: f32,
    rng: &mut ThreadRng,
    is_relink_classical: bool,
    mut builder: Builder,
) -> Vec<u8> {
    let mut tours = Vec::new();
    
    for i in 0..num_builds {
        // let tour = build_classical_03(p, alpha, rng);
        let tour = builder(p, alpha, rng);
        let mut visited = Set::with_capacity(p.len);
        visited.insert(0); visited.insert(1);
        for &v in tour.iter() { visited.insert(v as usize); }
        let tour = local_search(p, tour, &mut visited);
        // let status = p.eval_route(&tour);

        // if status.total_score > 1100 {
        //     println!("---------------------------");
        //     println!("tour   : {:?}", &tour);
        //     println!("score  : {}", status.total_score);
        //     println!("consume: {}", status.total_consume);
        //     println!("---------------------------");
        // }

        // assert_no_duplicates(p, &tour);
        println!("{:?}", i);
        tours.push(tour);
    }

    path_relink_all(p, tours, is_relink_classical, rng)
}

// pub fn grasp_megazord(p: &Problem, alpha: f32, rng: &mut ThreadRng) -> Vec<u8> {
//     let mut tours = Vec::new();
    
//     loop {
//         // let tour = build_classical_03(p, alpha, rng);
//         let tour = build02RI(p, alpha, rng, marginal_smart);
//         let mut visited = Set::with_capacity(p.len);
//         visited.insert(0); visited.insert(1);
//         for &v in tour.iter() { visited.insert(v as usize); }
//         let tour = local_search(p, tour, &mut visited);
//         let status = p.eval_route(&tour);

//         if status.total_score > 1100 {
//             println!("---------------------------");
//             println!("tour   : {:?}", &tour);
//             println!("score  : {}", status.total_score);
//             println!("consume: {}", status.total_consume);
//             println!("---------------------------");
//         }

//         assert_no_duplicates(p, &tour);
//         tours.push(tour);
//     }



//     todo!()
// }

// pub fn build_diverse(p: &Problem, alpha: f32, rng: &mut ThreadRng) -> Vec<u8> {
//     let mut tour      = Vec::with_capacity(p.len);
//     let mut visited = Set::with_capacity(p.len);
//     visited.insert(0); visited.insert(1);

//     loop {
//         let s = match rng.random_range(0..3) {
//             0 => marginal_envy,
//             1 => total_envy,
//             _ => marginal_wise,
//         };

//         let candidates = select_candidates(p, &tour, alpha, &visited, s);
//         if candidates.is_empty() { break; }

//         let idx = rng.random_range(0..candidates.len());
//         let choosen = &candidates[idx];
//         tour.insert(choosen.k, choosen.u);
//         visited.insert(choosen.u as usize);
//     }

//     tour
// }


fn assert_no_duplicates(p: &Problem, tour: &[u8]) {
    let mut seen = Set::with_capacity(p.len);

    for &u in tour {
        assert!(
            !seen.contains(u as usize),
            "duplicate vertex {u}: {tour:?}"
        );

        seen.insert(u as usize);
    }
}

use super::experiment::Result as ER;

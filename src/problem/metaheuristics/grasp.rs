
use fixedbitset::FixedBitSet as Set;

mod strategy;
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


pub fn grasp_classical(p: &Problem, alpha: f32, rng: &mut ThreadRng, mut builder: Builder) -> Vec<u8> {
    let mut tours = Vec::new();
    
    loop {
        // let tour = build_classical_03(p, alpha, rng);
        let tour = builder(p, alpha, rng, marginal_smart);
        let mut visited = Set::with_capacity(p.len);
        visited.insert(0); visited.insert(1);
        for &v in tour.iter() { visited.insert(v as usize); }
        let tour = local_search(p, tour, &mut visited);
        let status = p.eval_route(&tour);

        if status.total_score > 1100 {
            println!("---------------------------");
            println!("tour   : {:?}", &tour);
            println!("score  : {}", status.total_score);
            println!("consume: {}", status.total_consume);
            println!("---------------------------");
        }

        assert_no_duplicates(p, &tour);
        tours.push(tour);
    }



    todo!()
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


use fixedbitset::FixedBitSet as Set;

mod strategy;
use strategy::*;
use crate::problem::Problem;


pub mod local_search;
use local_search::local_search;
use rand::*;
use rand::rngs::ThreadRng;


pub fn grasp_classical(p: &Problem, alpha: f32, rng: &mut ThreadRng) -> Vec<u8> {
    let mut tours = Vec::new();
    
    loop {
        let tour = build_classical(p, alpha, rng);
        let mut visited = Set::with_capacity(p.len);
        visited.insert(0); visited.insert(1);
        let tour = local_search(p, tour, &mut visited);
        let status = p.eval_route(&tour);

        println!("---------------------------");
        println!("tour   : {:?}", &tour);
        println!("score  : {}", status.total_score);
        println!("consume: {}", status.total_consume);
        println!("---------------------------");

        tours.push(tour);
    }



    todo!()
}

pub fn build_classical(p: &Problem, alpha: f32, rng: &mut ThreadRng) -> Vec<u8> {
    let mut tour      = Vec::with_capacity(p.len);
    let mut visited = Set::with_capacity(p.len);
    visited.insert(0); visited.insert(1);

    loop {
        let candidates = select_candidates(p, &tour, alpha, &visited, marginal_smart);
        if candidates.is_empty() { break; }

        let idx = rng.random_range(0..candidates.len());
        let choosen = &candidates[idx];
        tour.insert(choosen.k, choosen.u);
        visited.insert(choosen.u as usize);
    }

    tour
}

pub fn select_candidates(
    p        : &Problem,
    tour     : &[u8],
    alpha    : f32,
    visited  : &Set,
    conversor: Conversor,
) -> Vec<Candidate> {
    let mut new_tour = Vec::with_capacity(tour.len() + 1);
    let mut candidates = Vec::with_capacity(tour.len() + 1);

    for k in 0..=tour.len() {
        for u in visited.zeroes() {
            new_tour.clear();
            new_tour.extend_from_slice(tour);
            new_tour.insert(k, u as u8);

            let status = p.eval_route(&new_tour);
            if status.total_consume > p.tmax { continue; }

            let score_gain = p.scores[u] as f32;
            let prev = if k == 0 { 0usize } else { tour[k-1] as usize };
            let next = if k == tour.len() { 1usize } else { tour[k] as usize };

            let cost_increase = p.costs[(prev, u)]    as f32
                              + p.costs[(u, next)]    as f32
                              - p.costs[(prev, next)] as f32;

            candidates.push(Candidate {
                u: u as u8, k, status, score_gain, cost_increase,
            })
        }
    }


    let Some(best) = candidates.iter()
        .max_by(|l, r| conversor(l).total_cmp(&conversor(r))) else {
        return Vec::new();
    };

    let best = best.clone();
    let candidates = candidates.into_iter()
        .filter(|c| {
            let c_ = conversor(c);
            let b_ = conversor(&best) * alpha;
            c_ >= b_
        }).collect();

    candidates
}

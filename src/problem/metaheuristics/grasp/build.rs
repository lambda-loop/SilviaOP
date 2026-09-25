
pub mod build01; pub use build01::build01;
pub mod build02; pub use build02::build02;
pub mod build03; pub use build03::build03;
pub mod build04; pub use build04::build04;

pub mod build01R; pub use build01R::build01R;
pub mod build02R; pub use build02R::build02R;
pub mod build03R; pub use build03R::build03R;
pub mod build04R; pub use build04R::build04R;

pub mod build01I; pub use build01I::build01I;
pub mod build02I; pub use build02I::build02I;
pub mod build03I; pub use build03I::build03I;
pub mod build04I; pub use build04I::build04I;

pub mod build01RI; pub use build01RI::build01RI;
pub mod build02RI; pub use build02RI::build02RI;
pub mod build03RI; pub use build03RI::build03RI;
pub mod build04RI; pub use build04RI::build04RI;

use rand::*;
use rand::rngs::ThreadRng;
use fixedbitset::FixedBitSet as Set;

use super::strategy::*;

use crate::problem::Problem;

use super::{Conversor, Candidate, marginal_smart};

type Spreader = Box<dyn FnMut(
    &Problem       , // p 
    &[u8]          , // tour
    f32            , // alpha 
    &Set           , // visited
    Conversor      , // conversor
    &mut ThreadRng , // rng
) -> Vec<Candidate>>;

type Filter = Box<dyn FnMut(
    &Problem       , // p 
    Vec<Candidate> , // candidates 
    f32            , // alpha 
    &Set           , // visited
    Conversor      , // conversor
    &mut ThreadRng , // rng
) -> Vec<Candidate>>;

type Picker = Box<dyn FnMut(Vec<Candidate>, &mut ThreadRng) -> Candidate>;

pub type Builder = Box<dyn FnMut(
    &Problem,
    f32,
    &mut ThreadRng,
     Conversor,
    // Spreader,
    // Filter,
    //  Picker,
) -> Vec<u8>>;

pub fn build_classical(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
    _conversor: Conversor,
    mut spreader: Spreader,
    mut filter: Filter,
    mut picker: Picker,
     
) -> Vec<u8> {
    let mut tour    = Vec::with_capacity(p.len);
    let mut visited = Set::with_capacity(p.len);
    visited.insert(0); visited.insert(1);

    loop {
        let candidates = spreader(p, &tour, alpha, &visited, marginal_smart, rng);
        if candidates.is_empty() { break; }

        let candidates = filter(p, candidates, alpha, &visited, marginal_smart, rng);
        // if candidates.is_empty() { break; }

        // let idx = rng.random_range(0..candidates.len());
        let picked = picker(candidates, rng);
        tour.insert(picked.k, picked.u);
        visited.insert(picked.u as usize);
    }

    tour
}

#[allow(non_snake_case)]
pub fn build_ranD(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
    // conversor: Conversor,
    mut spreader: Spreader,
    mut filter: Filter,
    mut picker: Picker,
     
) -> Vec<u8> {
    let mut tour    = Vec::with_capacity(p.len);
    let mut visited = Set::with_capacity(p.len);
    visited.insert(0); visited.insert(1);

    loop {
        let conversor = random_conversor(rng);
        let candidates = spreader(p, &tour, alpha, &visited, conversor, rng);
        if candidates.is_empty() { break; }

        let candidates = filter(p, candidates, alpha, &visited, conversor, rng);
        // if candidates.is_empty() { break; }

        // let idx = rng.random_range(0..candidates.len());
        let picked = picker(candidates, rng);
        tour.insert(picked.k, picked.u);
        visited.insert(picked.u as usize);
    }

    tour
}

pub fn build_rand_(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
    // conversor: Conversor,
    mut spreader: Spreader,
    mut filter: Filter,
    mut picker: Picker,
     
) -> Vec<u8> {
    let mut tour    = Vec::with_capacity(p.len);
    let mut visited = Set::with_capacity(p.len);
    visited.insert(0); visited.insert(1);

    loop {
        let conversor = random_conversor(rng);
        let candidates = spreader(p, &tour, alpha, &visited, marginal_smart, rng);
        if candidates.is_empty() { break; }

        let candidates = filter(p, candidates, alpha, &visited, marginal_smart, rng);
        // if candidates.is_empty() { break; }

        // let idx = rng.random_range(0..candidates.len());
        let picked = picker(candidates, rng);
        tour.insert(picked.k, picked.u);
        visited.insert(picked.u as usize);
    }

    tour
}


pub fn build__rand(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
    // conversor: Conversor,
    mut spreader: Spreader,
    mut filter: Filter,
    mut picker: Picker,
     
) -> Vec<u8> {
    let mut tour    = Vec::with_capacity(p.len);
    let mut visited = Set::with_capacity(p.len);
    visited.insert(0); visited.insert(1);

    loop {
        let candidates = spreader(p, &tour, alpha, &visited, marginal_smart, rng);
        if candidates.is_empty() { break; }

        let conversor = random_conversor(rng);
        let candidates = filter(p, candidates, alpha, &visited, conversor, rng);
        // if candidates.is_empty() { break; }

        // let idx = rng.random_range(0..candidates.len());
        let picked = picker(candidates, rng);
        tour.insert(picked.k, picked.u);
        visited.insert(picked.u as usize);
    }

    tour
}

pub fn build_rand_rand(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
    // conversor: Conversor,
    mut spreader: Spreader,
    mut filter: Filter,
    mut picker: Picker,
     
) -> Vec<u8> {
    let mut tour    = Vec::with_capacity(p.len);
    let mut visited = Set::with_capacity(p.len);
    visited.insert(0); visited.insert(1);

    loop {
        let conversor = random_conversor(rng);
        let candidates = spreader(p, &tour, alpha, &visited, conversor, rng);
        if candidates.is_empty() { break; }

        let conversor = random_conversor(rng);
        let candidates = filter(p, candidates, alpha, &visited, conversor, rng);
        // if candidates.is_empty() { break; }

        // let idx = rng.random_range(0..candidates.len());
        let picked = picker(candidates, rng);
        tour.insert(picked.k, picked.u);
        visited.insert(picked.u as usize);
    }

    tour
}


pub fn select_candidates_number(
    p        : &Problem,
    tour     : &[u8],
    alpha    : f32,
    visited  : &Set,
    conversor: Conversor,
    rng      : &mut ThreadRng,
) -> Vec<Candidate> {
    let mut new_tour = Vec::with_capacity(tour.len() + 1);
    let mut candidates = Vec::with_capacity(tour.len() + 1);

    for u in visited.zeroes() {
        let mut u_insertions = Vec::with_capacity(tour.len());
        for k in 0..=tour.len() {
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

            u_insertions.push(Candidate {
                u: u as u8, k, status, score_gain, cost_increase,
            })
        }
        if let Some(best) = u_insertions.into_iter().max_by(|l, r| conversor(l).total_cmp(&conversor(r))) {
            candidates.push(best);
        }
    }

    let candidates_len = candidates.len() as usize;
    let number = (candidates_len as f32 * alpha) as usize;
    // TODO: fix slow
    // instead of adding, lets just remove len - number o.O

    for _ in 0..(candidates_len - number) {
        let i = rng.random_range(0..candidates.len());
        candidates.remove(i);
    }
    
    candidates
}

pub fn select_candidates_filter(
    p        : &Problem,
    tour     : &[u8],
    alpha    : f32,
    visited  : &Set,
    conversor: Conversor,
    _        : &mut ThreadRng,
) -> Vec<Candidate> {
    let mut new_tour = Vec::with_capacity(tour.len() + 1);
    let mut candidates = Vec::with_capacity(tour.len() + 1);

    for u in visited.zeroes() {
        let mut u_insertions = Vec::with_capacity(tour.len());
        for k in 0..=tour.len() {
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

            u_insertions.push(Candidate {
                u: u as u8, k, status, score_gain, cost_increase,
            })
        }
        if let Some(best) = u_insertions.into_iter().max_by(|l, r| conversor(l).total_cmp(&conversor(r))) {
            candidates.push(best);
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


// pickers:

// must ensure that the cs is never empty!
pub fn pick_best_by(cs: Vec<Candidate>, conv: Conversor) -> Candidate {
    assert!(!cs.is_empty());
    cs.into_iter().max_by(|l, r| conv(l).total_cmp(&conv(r))).unwrap()
}

pub fn pick_random_in(cs: Vec<Candidate>, rng: &mut ThreadRng) -> Candidate {
    assert!(!cs.is_empty());
    let idx = rng.random_range(0..cs.len());
    cs[idx].clone()
}

fn random_conversor(rng: &mut ThreadRng) -> Conversor {
    match rng.random_range(0..3) {
        0 => marginal_envy,
        1 => total_envy,
        _ => marginal_wise,
    }
}


// use rand::*;
// use rand::rngs::ThreadRng;
// use fixedbitset::FixedBitSet as Set;

// use super::strategy::*;

// use crate::problem::Problem;

// use super::{Conversor, Candidate, marginal_smart};


use fixedbitset::FixedBitSet as Set;
use rand::*;
use rand::rngs::ThreadRng;

use crate::problem::{Problem, metaheuristics::OutOfCredits};

use super::strategy::{
    Candidate,
    Conversor,
    marginal_smart,

    total_envy,
    marginal_envy,
    marginal_wise,
};

pub type Builder = fn(
    &Problem,
    &mut usize,
    usize,
    f32,
    &mut ThreadRng
) -> Result<Vec<u8>, OutOfCredits>;

use crate::consume_credit;

pub fn build_classical(
    p: &Problem,
    used: &mut usize,
    budget: usize,
    alpha: f32,
    rng: &mut ThreadRng,
) -> Result<Vec<u8>, OutOfCredits> {
    let mut tour = Vec::with_capacity(p.len);

    let mut visited = Set::with_capacity(p.len);
    visited.insert(0);
    visited.insert(1);

    loop {
        let candidates =
            select_candidates(
                p,
                &tour,
                used,
                budget,
                alpha,
                &visited,
                marginal_smart,
            )?;

        if candidates.is_empty() {
            break;
        }

        let idx = rng.random_range(0..candidates.len());
        let chosen = &candidates[idx];

        tour.insert(chosen.k, chosen.u);
        visited.insert(chosen.u as usize);
    }

    Ok(tour)
}

pub fn build_rand(
    p: &Problem,
    used: &mut usize,
    budget: usize,
    alpha: f32,
    rng: &mut ThreadRng,
) -> Result<Vec<u8>, OutOfCredits> {
    let mut tour = Vec::with_capacity(p.len);

    let mut visited = Set::with_capacity(p.len);
    visited.insert(0);
    visited.insert(1);

    loop {
        let conversor = random_conversor(rng);
        let candidates =
            select_candidates(
                p,
                &tour,
                used,
                budget,
                alpha,
                &visited,
                conversor,
            )?;

        if candidates.is_empty() {
            break;
        }

        let idx = rng.random_range(0..candidates.len());
        let chosen = &candidates[idx];

        tour.insert(chosen.k, chosen.u);
        visited.insert(chosen.u as usize);
    }

    Ok(tour)
}

fn select_candidates(
    p: &Problem,
    tour: &[u8],
    used: &mut usize,
    budget: usize,
    alpha: f32,
    visited: &Set,
    conversor: Conversor,
) -> Result<Vec<Candidate>, OutOfCredits> {
    let mut new_tour =
        Vec::with_capacity(tour.len() + 1);

    let mut candidates = Vec::new();

    for u in visited.zeroes() {
        let mut best: Option<Candidate> = None;

        for k in 0..=tour.len() {
            new_tour.clear();
            new_tour.extend_from_slice(tour);
            new_tour.insert(k, u as u8);

            consume_credit!(used, budget);
            let status = p.eval_route(&new_tour);

            if status.total_consume > p.tmax {
                continue;
            }

            let prev =
                if k == 0 {
                    0
                } else {
                    tour[k - 1] as usize
                };

            let next =
                if k == tour.len() {
                    1
                } else {
                    tour[k] as usize
                };

            let cost_increase =
                  p.costs[(prev, u)] as f32
                + p.costs[(u, next)] as f32
                - p.costs[(prev, next)] as f32;

            let candidate = Candidate {
                u: u as u8,
                k,
                status,
                score_gain: p.scores[u] as f32,
                cost_increase,
            };

            match &best {
                None => {
                    best = Some(candidate);
                }

                Some(current)
                    if conversor(&candidate)
                        > conversor(current) =>
                {
                    best = Some(candidate);
                }

                _ => {}
            }
        }

        if let Some(best) = best {
            candidates.push(best);
        }
    }

    let Some(best) = candidates
        .iter()
        .max_by(|l, r| {
            conversor(l)
                .total_cmp(&conversor(r))
        })
    else {
        return Ok(Vec::new());
    };

    let threshold =
        alpha * conversor(best);

    Ok(candidates
        .into_iter()
        .filter(|c| {
            conversor(c) >= threshold
        })
        .collect())
}


fn random_conversor(rng: &mut ThreadRng) -> Conversor {
    match rng.random_range(0..3) {
        0 => total_envy,
        1 => marginal_wise,
        _ => marginal_envy,
    }
    
}

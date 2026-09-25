use rand::*;
use rand::rngs::ThreadRng;
use crate::problem::Problem;

use super::super::strategy::{
    marginal_envy,
    total_envy,
    marginal_wise,
};

use super::*;

fn random_conversor(rng: &mut ThreadRng) -> Conversor {
    match rng.random_range(0..3) {
        0 => marginal_envy,
        1 => total_envy,
        _ => marginal_wise,
    }
}

fn spread_c1ri(
    p: &Problem,
    tour: &[u8],
    _alpha: f32,
    visited: &Set,
    _conversor: Conversor,
    rng: &mut ThreadRng,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();

    for u in visited.zeroes() {
        let mut insertions = Vec::new();

        for k in 0..=tour.len() {
            let mut new_tour = tour.to_vec();
            new_tour.insert(k, u as u8);

            let status = p.eval_route(&new_tour);

            if status.total_consume > p.tmax {
                continue;
            }

            let prev = if k == 0 { 0 } else { tour[k - 1] as usize };
            let next = if k == tour.len() { 1 } else { tour[k] as usize };

            let cost_increase =
                  p.costs[(prev, u)] as f32
                + p.costs[(u, next)] as f32
                - p.costs[(prev, next)] as f32;

            insertions.push(Candidate {
                u: u as u8,
                k,
                status,
                score_gain: p.scores[u] as f32,
                cost_increase,
            });
        }

        if let Some(best) = insertions
            .into_iter()
            .max_by(|l, r| {
                let conv = random_conversor(rng);
                conv(l).total_cmp(&conv(r))
            })
        {
            candidates.push(best);
        }
    }

    candidates
}

fn filter_c1ri(
    _p: &Problem,
    candidates: Vec<Candidate>,
    alpha: f32,
    _visited: &Set,
    _conversor: Conversor,
    rng: &mut ThreadRng,
) -> Vec<Candidate> {
    let Some(best) = candidates
        .iter()
        .max_by(|l, r| {
            let conv = random_conversor(rng);
            conv(l).total_cmp(&conv(r))
        })
    else {
        return Vec::new();
    };

    let best = best.clone();

    candidates
        .into_iter()
        .filter(|c| {
            let conv = random_conversor(rng);

            conv(c) >= alpha * conv(&best)
        })
        .collect()
}

fn pick_c1ri(
    candidates: Vec<Candidate>,
    rng: &mut ThreadRng,
) -> Candidate {
    pick_random_in(candidates, rng)
}

#[allow(non_snake_case)]
pub fn build01RI(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
    _conversor: Conversor,
) -> Vec<u8> {
    build_classical(
        p,
        alpha,
        rng,
        marginal_envy,
        Box::new(spread_c1ri),
        Box::new(filter_c1ri),
        Box::new(pick_c1ri),
    )
}

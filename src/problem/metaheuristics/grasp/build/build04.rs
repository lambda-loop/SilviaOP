use rand::rngs::ThreadRng;
use crate::problem::Problem;

use super::super::strategy::{
    marginal_lazy as cost,
    marginal_smart as score_per_cost,
};

use super::*;

fn spread_c4(
    p: &Problem,
    tour: &[u8],
    _alpha: f32,
    visited: &Set,
    _conversor: Conversor,
    _rng: &mut ThreadRng,
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

            let prev = if k == 0 {
                0
            } else {
                tour[k - 1] as usize
            };

            let next = if k == tour.len() {
                1
            } else {
                tour[k] as usize
            };

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
            .max_by(|l, r| cost(l).total_cmp(&cost(r)))
        {
            candidates.push(best);
        }
    }

    candidates
}

fn filter_c4(
    _p: &Problem,
    mut candidates: Vec<Candidate>,
    alpha: f32,
    _visited: &Set,
    _conversor: Conversor,
    rng: &mut ThreadRng,
) -> Vec<Candidate> {
    if candidates.is_empty() {
        return candidates;
    }

    let len = candidates.len();

    // Explicitly ceil(alpha * |CL|) in C4.
    let number = ((len as f32 * alpha).ceil() as usize)
        .max(1)
        .min(len);

    while candidates.len() > number {
        let i = rng.random_range(0..candidates.len());
        candidates.remove(i);
    }

    candidates
}

fn pick_c4(
    candidates: Vec<Candidate>,
    _rng: &mut ThreadRng,
) -> Candidate {
    pick_best_by(candidates, score_per_cost)
}

pub fn build04(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    build_classical(
        p,
        alpha,
        rng,
        score_per_cost,
        Box::new(spread_c4),
        Box::new(filter_c4),
        Box::new(pick_c4),
    )
}

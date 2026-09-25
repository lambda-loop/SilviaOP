use rand::rngs::ThreadRng;
use crate::problem::Problem;

use super::super::strategy::{
    marginal_lazy as cost,
    marginal_smart as score_per_cost,
};

use super::*;

fn spread_c3(
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

        // Delta t_i = smallest insertion increment
        if let Some(best) = insertions
            .into_iter()
            .max_by(|l, r| cost(l).total_cmp(&cost(r)))
        {
            candidates.push(best);
        }
    }

    candidates
}

fn filter_c3(
    _p: &Problem,
    candidates: Vec<Candidate>,
    alpha: f32,
    _visited: &Set,
    conversor: Conversor,
    _rng: &mut ThreadRng,
) -> Vec<Candidate> {
    let Some(best) = candidates
        .iter()
        .max_by(|l, r| conversor(l).total_cmp(&conversor(r)))
    else {
        return Vec::new();
    };

    let threshold = alpha * conversor(best);

    candidates
        .into_iter()
        .filter(|c| conversor(c) >= threshold)
        .collect()
}

fn pick_c3(
    candidates: Vec<Candidate>,
    rng: &mut ThreadRng,
) -> Candidate {
    pick_random_in(candidates, rng)
}

pub fn build03(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    build_classical(
        p,
        alpha,
        rng,
        score_per_cost,
        Box::new(spread_c3),
        Box::new(filter_c3),
        Box::new(pick_c3),
    )
}

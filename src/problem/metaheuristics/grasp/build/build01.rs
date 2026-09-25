

use rand::rngs::ThreadRng;
use crate::problem::Problem;

use super::super::strategy::marginal_greedy as score;

use super::*;

fn spread_c1(
    p: &Problem,
    tour: &[u8],
    _alpha: f32,
    visited: &Set,
    _conversor: Conversor,
    _rng: &mut ThreadRng,
) -> Vec<Candidate> {
    let mut candidates = Vec::new();

    for u in visited.zeroes() {
        let mut best: Option<Candidate> = None;

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

            let candidate = Candidate {
                u: u as u8,
                k,
                status,
                score_gain: p.scores[u] as f32,
                cost_increase,
            };

            match &best {
                None => best = Some(candidate),

                Some(current)
                    if candidate.cost_increase < current.cost_increase =>
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

    candidates
}


fn filter_c1(
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

    let threshold = conversor(best) * alpha;

    candidates
        .into_iter()
        .filter(|c| conversor(c) >= threshold)
        .collect()
}


fn pick_c1(
    candidates: Vec<Candidate>,
    rng: &mut ThreadRng,
) -> Candidate {
    pick_random_in(candidates, rng)
}


pub fn build01(
    p: &Problem,
    alpha: f32,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    build_classical(
        p,
        alpha,
        rng,
        score,
        Box::new(spread_c1),
        Box::new(filter_c1),
        Box::new(pick_c1),
    )
}

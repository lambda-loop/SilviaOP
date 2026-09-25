
use std::collections::HashSet;

use super::{Conversor, Candidate};
use fixedbitset::FixedBitSet as Set;
use crate::problem::Problem;

pub fn path_relinking(
    p   : &Problem,
    from: &[u8],
    to  : &[u8],
    conversor: Conversor,
) -> Vec<u8> {
    let mut current_tour = from.to_vec();
    let mut best_tour    = from.to_vec();
    let mut best_score   = p.eval_route(&from).total_score;


    let mut to_add = {
        let mut vs = Vec::with_capacity(to.len());
        for &v in to.iter() {
            if !from.contains(&v) {
                vs.push(v);
            }
        }

        vs
    };

    let mut to_remove = {
        let mut vs = Vec::with_capacity(from.len());
        for &v in from.iter() {
            if !to.contains(&v) {
                vs.push(v);
            }
        }

        vs
    };
    
    to_add.sort_unstable_by(|&l, &r| {
        let l = p.scores[l as usize];
        let r = p.scores[r as usize];
        r.cmp(&l)
    });

    // note the pop used later. So its acctually in the reverse order.

    to_remove.sort_unstable_by(|&l, &r|{
        let l = p.scores[l as usize];
        let r = p.scores[r as usize];
        r.cmp(&l)
    });

    for u in to_add {
        let k = (0..=current_tour.len())
                .map(|k| Candidate::new(p, &current_tour, u as usize, k))
                .max_by(|l, r| conversor(l).total_cmp(&conversor(r)))
                .unwrap()
                .k;

        current_tour.insert(k, u);
        while p.eval_route(&current_tour).total_consume > p.tmax {
            if let Some(v)   = to_remove.pop() && 
               let Some(pos) = current_tour.iter().position(|&x| x == v) {
                    current_tour.remove(pos);
                
            } else {
                if let Some(pos) = current_tour.iter().position(|&x| x == u) {
                    current_tour.remove(pos);
                }
                break;
            }
        }
        let current_score = p.eval_route(&current_tour).total_score;
        if current_score > best_score {
            best_score = current_score;
            best_tour  = current_tour.clone();
        }
    }

    best_tour
}

pub fn path_relink_all(
    p: &Problem,
    tours: Vec<Vec<u8>>,
    is_classical: bool,
    rng: &mut ThreadRng,
    // conversor: Conversor,
) -> Vec<u8> {
    assert!(!tours.is_empty());

    // Remove tours exactly equal to each other.
    let tours: Vec<Vec<u8>> = tours
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // The best solution may already be one of the original tours.
    let mut best_tour = tours[0].clone();
    let mut best_score = p.eval_route(&best_tour).total_score;

    for tour in &tours {
        let score = p.eval_route(tour).total_score;

        if score > best_score {
            best_score = score;
            best_tour = tour.clone();
        }
    }

    // Every distinct unordered pair exactly once.
    for i in 0..tours.len() {
        for j in (i + 1)..tours.len() {
            // i -> j
            let conversor = if is_classical { marginal_lazy }
                else { random_conversor(rng) };
            let relinked =
                path_relinking(
                    p,
                    &tours[i],
                    &tours[j],
                    conversor,
                );

            let score =
                p.eval_route(&relinked).total_score;

            if score > best_score {
                best_score = score;
                best_tour = relinked;
            }

            // j -> i
            let conversor = if is_classical { marginal_lazy }
                else { random_conversor(rng) };
            let relinked =
                path_relinking(
                    p,
                    &tours[j],
                    &tours[i],
                    conversor,
                );

            let score =
                p.eval_route(&relinked).total_score;

            if score > best_score {
                best_score = score;
                best_tour = relinked;
            }
        }
    }

    best_tour
}

use rand::*;
use rand::rngs::ThreadRng;
use super::{total_envy, marginal_wise, marginal_envy, marginal_lazy};

fn random_conversor(rng: &mut ThreadRng) -> Conversor {
    match rng.random_range(0..3) {
        0 => total_envy,
        1 => marginal_wise,
        _ => marginal_envy,
    }
    
}

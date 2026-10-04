
use std::collections::HashSet;

use super::{Conversor, Candidate};
use fixedbitset::FixedBitSet as Set;
use crate::problem::{Problem, metaheuristics::OutOfCredits};

use crate::consume_credit;
pub fn path_relinking(
    p     : &Problem,
    used  : &mut usize,
    budget: usize,
    from: &[u8],
    to  : &[u8],
    conversor: Conversor,
) -> Result<Vec<u8>, OutOfCredits> {
    let mut current_tour = from.to_vec();
    let mut best_tour    = from.to_vec();
    consume_credit!(used, budget);
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
                .map(|k| Candidate::new(p, &current_tour, used, budget, u as usize, k))
                .collect::<Result<Vec<_>, OutOfCredits>>()?
                .into_iter()
                .max_by(|l, r| conversor(l).total_cmp(&conversor(r)))
                .unwrap()
                .k;

        let previous_tour = current_tour.clone();
        let previous_to_remove = to_remove.clone();
        current_tour.insert(k, u);
        while {
            consume_credit!(used, budget);
            p.eval_route(&current_tour).total_consume > p.tmax
        } {

            if let Some(v)   = to_remove.pop() && 
               let Some(pos) = current_tour.iter().position(|&x| x == v) {
                    current_tour.remove(pos);
                
            } else {
                   current_tour = previous_tour;
                   to_remove = previous_to_remove;
                   break;
            }
            
        }

        consume_credit!(used, budget);
        let current_score = p.eval_route(&current_tour).total_score;
        if current_score > best_score {
            best_score = current_score;
            best_tour  = current_tour.clone();
        }
    }

    Ok(best_tour)
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

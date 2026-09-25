

use crate::problem::Problem;
use fixedbitset::FixedBitSet as Set;

use std::cmp::Ordering;

// neighboorhood generators

// implements the article 2014 version
// 2opt
// exchange 

// let cmp = |l: &[u8], r: &[u8]| {
fn cmp(p: &Problem, l: &[u8], r: &[u8]) -> std::cmp::Ordering {
    let l_status = p.eval_route(&l);
    let r_status = p.eval_route(&r);
    if l_status.total_score < r_status.total_score {
        return Ordering::Less;
    } else if l_status.total_score == r_status.total_score {
        return r_status.total_consume.total_cmp(&l_status.total_consume);
    }

    return Ordering::Greater;
}


pub fn local_search(p: &Problem, mut tour: Vec<u8>, visited: &mut Set) -> Vec<u8> {

    let mut changed = true;
    while changed {
        changed = false;
        let two_opts = two_opt(p, &tour); 
        if let Some(best) = two_opts.into_iter().max_by(|l, r| cmp(p, l, r)) {
            if cmp(p, &best, &tour) == Ordering::Greater {
                tour = best;
                changed = true;
            }
        }
    }

    while exchange_inserts(p, &mut tour, visited) {}

    changed = true;
    while changed {
        changed = false;
        let two_opts = two_opt(p, &tour); 
        if let Some(best) = two_opts.into_iter().max_by(|l, r| cmp(p, l, r)) {
            if cmp(p, &best, &tour) == Ordering::Greater {
                tour = best;
                changed = true;
            }
        }
    }

    tour
}

struct ExchangeCtx {
    pub removed : usize,
    pub inserted: usize,
    pub tour: Vec<u8>,
}

// This one will keep going and going like the article version
pub fn exchange_inserts (
    p: &Problem,
    tour: &mut Vec<u8>,
    visited: &mut Set,
) -> bool {
    let mut changed = false;
    for k in 0..tour.len() {
        if let Some(exchange) = best_exchange_1_1_in(p, &tour, visited, k) &&
            let Ordering::Greater = cmp(p, &exchange.tour, &tour) {
                visited.insert(exchange.inserted);
                visited.remove(exchange.removed);
                *tour = exchange.tour;

                changed = true;

                let mut changed = true;
                while changed {
                    changed = false;
                    let insertions = insertion(p, &tour, visited); 
                    if let Some((inserted, best)) = insertions.into_iter().max_by(|(_, l), (_, r)| cmp(p, l, r)) {
                        if cmp(p, &best, &tour) == Ordering::Greater {
                            *tour = best;
                            visited.insert(inserted as usize);
                            changed = true;
                        }
                    }
                }
            }
    }

    changed
}

// (old, new, vec)
pub fn best_exchange_1_1_in(
    p: &Problem,
    tour: &[u8],
    visited: &Set,
    idx: usize
) -> Option<ExchangeCtx> {
    let mut neighborhood = Vec::with_capacity(visited.zeroes().count());

    for u in visited.zeroes() {
        let mut new_tour = tour.to_vec();
        let old = tour[idx];
        new_tour[idx] = u as u8;
        let status = p.eval_route(&new_tour);
        if status.total_consume > p.tmax { continue; }
        neighborhood.push((old as usize, u, new_tour));
    }

    let (removed, inserted, tour) = neighborhood.into_iter().max_by(|(_, _, l), (_, _, r)| cmp(p, l, r))?;
    Some(ExchangeCtx { removed, inserted, tour })
}


pub fn insertion(p: &Problem, tour: &[u8], visited: &Set) -> Vec<(u8, Vec<u8>)> {
    let cmp = |l: &[u8], r: &[u8]| {
        let l_status = p.eval_route(&l);
        let r_status = p.eval_route(&r);
        if l_status.total_score < r_status.total_score {
            return Ordering::Less;
        } else if l_status.total_score == r_status.total_score {
            return r_status.total_consume.total_cmp(&l_status.total_consume);
        }

        return Ordering::Greater;
    };

    let tour_len = tour.len();
    let zeroes = visited.zeroes().count();
    let mut neighborhood = Vec::with_capacity(tour_len * zeroes);

    for i in 0..=tour_len {
        let mut i_tours = Vec::with_capacity(zeroes);
        for u in visited.zeroes() {
            let mut new_tour = tour.to_vec();
            new_tour.insert(i, u as u8);
            let status = p.eval_route(&new_tour);
            if status.total_consume > p.tmax { continue; }
            i_tours.push((u as u8, new_tour));
        }

        if let Some(best) = i_tours.into_iter().max_by(|(_, l), ( _, r)| cmp(l, r)) {
            neighborhood.push(best);
        }
        
    }

    neighborhood
}

pub fn two_opt(
    p: &Problem,
    tour: &[u8],
) -> Vec<Vec<u8>> {
    let tour_len = tour.len();
    let mut neighborhood = Vec::new();

    for i in 0..tour_len {
        for j in i + 1..tour_len {
            let mut new_tour = tour.to_vec();

            new_tour[i..=j].reverse();

            let status = p.eval_route(&new_tour);

            if status.total_consume > p.tmax {
                continue;
            }

            neighborhood.push(new_tour);
        }
    }

    neighborhood
}

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

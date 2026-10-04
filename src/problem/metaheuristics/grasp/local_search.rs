

use crate::problem::Problem;
use fixedbitset::FixedBitSet as Set;

use std::cmp::Ordering;

use crate::consume_credit;
use super::super::OutOfCredits;

fn cmp(
    p: &Problem,
    used: &mut usize,
    budget: usize,
    l: &[u8],
    r: &[u8]
) -> Result<std::cmp::Ordering, OutOfCredits> {
    consume_credit!(used, budget);
    let l_status = p.eval_route(&l);
    consume_credit!(used, budget);
    let r_status = p.eval_route(&r);
    if l_status.total_score < r_status.total_score {
        return Ok(Ordering::Less);
    } else if l_status.total_score == r_status.total_score {
        return Ok(r_status.total_consume.total_cmp(&l_status.total_consume));
    }

    return Ok(Ordering::Greater);
}


pub fn local_search(
    p: &Problem,
    used: &mut usize,
    budget: usize,
    mut tour: Vec<u8>,
    visited: &mut Set
) -> Result<Vec<u8>, OutOfCredits> {
    let mut changed = true;
    while changed {
        changed = false;
        let two_opts = two_opt(p, used, budget, &tour)?; 
        let iter = two_opts.into_iter();
        if let Some(best) =  try_max_by(iter, |l, r| cmp(p, used, budget, l, r))? {
            if let Ordering::Greater = cmp(p, used, budget, &best, &tour)? {
                tour = best;
                changed = true;
            }
        }
    }

    while exchange_inserts(p, used, budget, &mut tour, visited)? {}

    changed = true;
    while changed {
        changed = false;
        let two_opts = two_opt(p, used, budget, &tour)?; 
        let iter = two_opts.into_iter();
        if let Some(best) =  try_max_by(iter, |l, r| cmp(p, used, budget, l, r))? {
            if let Ordering::Greater = cmp(p, used, budget, &best, &tour)? {
                tour = best;
                changed = true;
            }
        }
    }

    Ok(tour)
}

struct ExchangeCtx {
    pub removed : usize,
    pub inserted: usize,
    pub tour: Vec<u8>,
}

// This one will keep going and going like the article version
pub fn exchange_inserts (
    p: &Problem,
    used: &mut usize,
    budget: usize,
    tour: &mut Vec<u8>,
    visited: &mut Set,
) -> Result<bool, OutOfCredits> {
    let mut changed = false;
    for k in 0..tour.len() {
        if let Some(exchange) = best_exchange_1_1_in(p, used, budget, &tour, visited, k)? &&
            let Ordering::Greater = cmp(p, used, budget, &exchange.tour, &tour)? {
                visited.insert(exchange.inserted);
                visited.remove(exchange.removed);
                *tour = exchange.tour;

                changed = true;

                let mut changed = true;
                while changed {
                    changed = false;
                    let insertions = insertion(p, used, budget, &tour, visited)?; 
                    let iter = insertions.into_iter();
                    if let Some((inserted, best)) = try_max_by(iter, |(_, l), (_, r)| cmp(p, used, budget, l, r))? {
                        if cmp(p, used, budget, &best, &tour)? == Ordering::Greater {
                            *tour = best;
                            visited.insert(inserted as usize);
                            changed = true;
                        }
                    }
                }
            }
    }

    Ok(changed)
}

// (old, new, vec)
pub fn best_exchange_1_1_in(
    p: &Problem,
    used: &mut usize,
    budget: usize,
    tour: &[u8],
    visited: &Set,
    idx: usize
) -> Result<Option<ExchangeCtx>, OutOfCredits> {
    let mut neighborhood = Vec::with_capacity(visited.zeroes().count());

    for u in visited.zeroes() {
        let mut new_tour = tour.to_vec();
        let old = tour[idx];
        new_tour[idx] = u as u8;
        consume_credit!(used, budget);
        let status = p.eval_route(&new_tour);
        if status.total_consume > p.tmax { continue; }
        neighborhood.push((old as usize, u, new_tour));
    }

    let iter = neighborhood.into_iter();
    if let Some((removed, inserted, tour)) = try_max_by(iter, |(_, _, l), (_, _, r)| cmp(p, used, budget, l, r))? {
        Ok(Some(ExchangeCtx { removed, inserted, tour }))
    }
    else { Ok(None) }
}


pub fn insertion(
    p: &Problem,
    used: &mut usize,
    budget: usize,
    tour: &[u8],
    visited: &Set
) -> Result<Vec<(u8, Vec<u8>)>, OutOfCredits> {
    let mut cmp = |used: &mut usize, l: &[u8], r: &[u8]| {
        consume_credit!(used, budget);
        let l_status = p.eval_route(&l);
        consume_credit!(used, budget);
        let r_status = p.eval_route(&r);
        if l_status.total_score < r_status.total_score {
            return Ok(Ordering::Less);
        } else if l_status.total_score == r_status.total_score {
            return Ok(r_status.total_consume.total_cmp(&l_status.total_consume));
        }

        return Ok(Ordering::Greater);
    };

    let tour_len = tour.len();
    let zeroes = visited.zeroes().count();
    let mut neighborhood = Vec::with_capacity(tour_len * zeroes);

    for i in 0..=tour_len {
        let mut i_tours = Vec::with_capacity(zeroes);
        for u in visited.zeroes() {
            let mut new_tour = tour.to_vec();
            new_tour.insert(i, u as u8);

            consume_credit!(used, budget);
            let status = p.eval_route(&new_tour);
            if status.total_consume > p.tmax { continue; }
            i_tours.push((u as u8, new_tour));
        }

        let best = i_tours
            .into_iter()
            .try_fold(None, |best, candidate| -> Result<_, OutOfCredits> {
                match best {
                    None => Ok(Some(candidate)),
                    
                    Some(current) => {
                        if cmp(used, &current.1, &candidate.1)? == Ordering::Less {
                            Ok(Some(candidate))
                        } else {
                            Ok(Some(current))
                        }
                    }
                }
            })?;
        
        if let Some(best) = best {
            neighborhood.push(best);
        }        
    }

    Ok(neighborhood)
}

pub fn two_opt(
    p: &Problem,
    used: &mut usize,
    budget: usize,
    tour: &[u8],
) -> Result<Vec<Vec<u8>>, OutOfCredits> {
    let tour_len = tour.len();
    let mut neighborhood = Vec::new();

    for i in 0..tour_len {
        for j in i + 1..tour_len {
            let mut new_tour = tour.to_vec();

            new_tour[i..=j].reverse();

            consume_credit!(used, budget);
            let status = p.eval_route(&new_tour);
            if status.total_consume > p.tmax {
                continue;
            }

            neighborhood.push(new_tour);
        }
    }

    Ok(neighborhood)
}


fn try_max_by<T, I, F>(
    iter: I,
    mut cmp: F,
) -> Result<Option<T>, OutOfCredits>
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T, &T) -> Result<Ordering, OutOfCredits>,
{
    iter.into_iter()
        .try_fold(None, |best, candidate| -> Result<_, OutOfCredits> {
            match best {
                None => Ok(Some(candidate)),

                Some(current) => {
                    if cmp(&current, &candidate)? == Ordering::Less {
                        Ok(Some(candidate))
                    } else {
                        Ok(Some(current))
                    }
                }
            }
        })
}

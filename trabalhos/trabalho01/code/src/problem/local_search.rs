//
use crate::problem::Problem;

use std::collections::HashSet;

use super::route::RouteStatus;
use std::cmp::Ordering;

pub fn simple_best_insertion_by(
    p: &Problem,
    r: &mut Vec<u8>,
    unvisited: &mut HashSet<u8>,
    strategy: fn(&RouteStatus, &RouteStatus) -> Ordering,
) -> bool {
    let mut best: Option<(u8, usize, RouteStatus)> = None;

    let mut new_r = Vec::with_capacity(r.len() + 1);
    for k in 0..=r.len() {
        for &u in unvisited.iter() {
            new_r.clear();
            new_r.extend_from_slice(r);
            new_r.insert(k, u);

            let sts = p.eval_route(&new_r);

            if sts.total_consume > p.tmax {
                continue;
            }

            match &best {
                None => best = Some((u, k, sts)),
                Some((_, _, best_sts)) => {
                    if strategy(&sts, best_sts) == Ordering::Less {
                        best = Some((u, k, sts));
                    }
                }
            }
        }
    }

    match best {
        None => false,
        Some((u, k, _)) => {
            r.insert(k, u);
            unvisited.remove(&u);
            true
        }
    }
}

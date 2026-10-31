
// everything will be first improvement here

use crate::{
    Problem, problem::{
        tour::*,
        TourStatus, 
    },
};

use super::Fitness;
// intense
pub type LocalSearch = fn(
    tour   : &mut Tour,
    used   : &mut u64 ,
    budget : u64      ,
    problem: &Problem ,
    fitness: Fitness  ,
) -> TourStatus; // (fit, cost)

pub fn remotion(
    tour   : &mut Tour ,
    status : TourStatus,
    fit    : f64       , 
    used   : &mut u64  ,
    budget : u64       ,
    problem: &Problem  ,
    fitness: Fitness   ,
) -> Option<(f64, TourStatus)> {
    let tour_len = tour.len();
    for i in 0..tour_len {
        if *used >= budget {
            break;
        }

        let status_ = compute_with_remotion (
            tour,
            status,
            i as usize,
            used,
            problem
        );

        let fit_ = (fitness)(status_);
        if fit_ > fit {
            tour.remove_idx(i as usize);
            return Some((fit_, status_))
        }
    }

    None
}

fn compute_with_remotion(
    tour   : &Tour,
    status : TourStatus,
    idx    : usize,
    used   : &mut u64,
    p      : &Problem,
) -> TourStatus {
    let tour_len = tour.len() as usize;
    let removed_v = tour[idx] as usize;

    let score = status.score
        - p.scores[removed_v] as u32;

    let (r, s) = (
        if idx == 0 {
            0
        } else {
            tour[idx - 1] as usize
        },
        if idx + 1 == tour_len {
            1
        } else {
            tour[idx + 1] as usize
        },
    );

    let cost = status.cost
        - p.costs[(r, removed_v)]
        - p.costs[(removed_v, s)]
        + p.costs[(r, s)];

    *used += 1;

    TourStatus { score, cost }
}


// everything will be best* improvement here

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

pub fn insertion (
    tour   : &mut Tour ,
    status : TourStatus,
    fit    : f64       , 
    used   : &mut u64  ,
    budget : u64       ,
    problem: &Problem  ,
    fitness: Fitness   ,
) -> Option<(TourStatus, f64)> {
    let mut best_fit = fit;
    let mut best_ts  = status;
    let mut best_idx = None;
    

    for v in tour.visited().zeroes() {
        if *used >= budget { break }
        let Some((idx, ts, fit_))
            = find_best_insertion_idx(
                tour   ,
                status ,
                fit    ,
                used   ,
                budget ,
                problem,
                fitness,
                v      ,
            ) else { continue };

        if fit_ > best_fit {
            best_fit = fit_;
            best_idx = Some((v, idx));
            best_ts  = ts;
        }
    }

    match best_idx {
        None => None,
        Some((v, idx)) => {
            tour.insert_in(v as u16, idx);
            Some((best_ts, best_fit))
        },
    }
}

// only cares about cost
fn find_best_insertion_idx (
    tour   : &Tour ,
    status : TourStatus,
    fit    : f64       , 
    used   : &mut u64  ,
    budget : u64       ,
    problem: &Problem  ,
    fitness: Fitness   ,

    v: usize           ,
) -> Option<(usize, TourStatus, f64)> {
    let mut best_fit = fit;
    let mut best_idx = None;
    let mut best_ts  = status;

    let tour_len = tour.len() as usize;
    for i in 0..=tour_len {
        if *used >= budget { break }
        let ts = compute_with_insertion_in(
            tour,
            status,
            i,
            v,
            used,
            problem
        );

        let fit_ = (fitness)(ts);
        if fit_ > best_fit {
            best_fit = fit_;
            best_ts  = ts;
            best_idx = Some(i);
        }

    }

    match best_idx {
        None => None,
        Some(i) => Some((i, best_ts, best_fit)),
    }
}

fn compute_with_insertion_in (
    tour   : &Tour,
    status : TourStatus,

    idx    : usize,
    v      : usize,
    used   : &mut u64,

    p      : &Problem,
) -> TourStatus {
    let score = status.score
        + p.scores[v] as u32;

    let (r, s) = (
        if idx == 0 {
            0
        } else {
            tour[idx - 1] as usize
        },
        if idx == tour.len() as usize {
            1
        } else {
            tour[idx] as usize
        },
    );

    let cost = status.cost
        + p.costs[(r, v)]
        + p.costs[(v, s)]
        - p.costs[(r, s)];

    *used += 1;
    TourStatus { score, cost }
}

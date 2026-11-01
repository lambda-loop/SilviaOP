
// everything will be first improvement here
mod remotion;
mod insertion;
mod sweeping;

use remotion::remotion;
use insertion::insertion;
use sweeping::sweeping;

use crate::{
    Problem, problem::{
        tour::*,
        TourStatus, 
    },
};

use super::Fitness;
// intense
pub type LocalSearch = fn(
    tour   : &mut Tour ,
    status : TourStatus,
    fit    : f64       ,
    used   : &mut u64  ,
    budget : u64       ,
    problem: &Problem  ,
    fitness: Fitness   ,
) -> (TourStatus, f64); // (status, fit)


pub fn canonical(
    tour   : &mut Tour ,
    mut status : TourStatus,
    mut fit    : f64       ,
    used   : &mut u64  ,
    budget : u64       ,
    problem: &Problem  ,
    fitness: Fitness   ,
) -> (TourStatus, f64) {
    let tmax = problem.tmax;

    if status.cost > tmax {
        if let Some((fit_, ts)) 
            = remotion(
            tour,
            status,
            fit,
            used,
            budget,
            problem,
            fitness
            ) {
                fit = fit_;
                status = ts;
            }

    } else {
        if let Some((ts, fit_)) 
            = insertion(
            tour,
            status,
            fit,
            used,
            budget,
            problem,
            fitness
            ) {
                fit = fit_;
                status = ts;
            }
    }

    (status, fit)
}

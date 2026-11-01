
// everything will be first improvement here
mod remotion;
mod insertion;
mod sweeping;

use fixedbitset::FixedBitSet as BitSet;
use grid::Grid;
use rand::{
    *,
    rngs::SmallRng,
};

use remotion::remotion;
use insertion::insertion;
use sweeping::sweeping;

use crate::{
    Problem, problem::{
        tour::*,
        TourStatus, 
    },
};

pub type TspSearch = fn(
    tour   : &mut [Point],
    rng: &mut SmallRng,
    costs: &Grid<f64>,
) -> f64;

use super::Fitness;
// intense
pub type LocalSearch = fn(
    tour    : &mut Tour ,
    status  : TourStatus,
    fit     : f64       ,
    rng: &mut SmallRng,
    used    : &mut u64  ,
    budget  : u64       ,
    problem : &Problem  ,
    fitness : Fitness   ,
    searcher: TspSearch , 
) -> (TourStatus, f64); // (status, fit)

#[derive(Clone, Copy)]
enum Trys {
    Working,
    FailedOnce,
    GiveUp,
}

impl Trys {
    fn reset() -> Self {
        Trys::Working
    }

    fn next(self) -> Self {
        match self {
            Trys::Working => Trys::FailedOnce,
            _             => Trys::GiveUp    ,
        }
        
    }
}

pub fn canonical(
    tour   : &mut Tour ,
    mut status : TourStatus,
    mut fit    : f64       ,
    rng: &mut SmallRng,
    used   : &mut u64  ,
    budget : u64       ,
    problem: &Problem  ,
    fitness: Fitness   ,
    searcher: TspSearch , 
) -> (TourStatus, f64) {
    let tmax = problem.tmax;
    let mut trys = Trys::Working;

    loop {
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
                    trys = Trys::reset();
            } else {
                if let Trys::GiveUp = trys { break }
                else { trys = trys.next() }
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

            trys = Trys::reset();
            } else {
                if let Trys::GiveUp = trys { break }
                else { trys = trys.next() }
            }
        }

        if *used >= budget { break }
        if sweeping(tour.as_mut_slice(), &problem.costs) {
            status.cost = 
                (searcher)(tour.as_mut_slice(), rng, &problem.costs);
            *used += 1;
            fit = (fitness)(status);
            trys = Trys::reset();
        }
    }

    (status, fit)
}

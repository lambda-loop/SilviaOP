
// everything will be first improvement here
mod remotion;

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

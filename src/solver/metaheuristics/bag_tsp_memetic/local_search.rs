

use crate::{
    Problem, problem::{
        tour::*,
        TourStatus, 
    },
};

// intense
pub type LocalSearch = fn(
    tour   : &mut Tour,
    used   : &mut u64,
    budget : u64,
    problem: &Problem,
) -> TourStatus; // (fit, cost)

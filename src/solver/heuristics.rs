
pub mod naive_greedy;
use crate::{
    Problem, Tour,
};

pub type Heuristic =
    fn(&Problem) -> Tour;

use crate::problem::RouteStatus;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Candidate {
    pub u: u8,
    pub k: usize,

    pub status: RouteStatus,

    pub score_gain: f32,
    pub cost_increase: f32,
}

pub type Conversor = fn(&Candidate) -> f32;

// #[derive(Clone, Copy)]
// pub struct NamedStrategy {
//     pub name: &'static str,
//     pub cmp: StrategyFn,
// }

pub fn marginal_smart(
    c: &Candidate,
) -> f32 {
    c.score_gain / c.cost_increase
}

// Random Selection:
pub fn marginal_envy(
    c: &Candidate,
) -> f32 {
    (c.score_gain * c.score_gain) / c.cost_increase
}

pub fn total_envy(
    c: &Candidate,
) -> f32 {
    (c.status.total_score * c.status.total_score)
        / c.status.total_consume
}

pub fn marginal_wise(
    c: &Candidate,
) -> f32 {
    c.score_gain / (c.cost_increase * c.cost_increase)
}

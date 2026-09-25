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


use crate::problem::Problem;

impl Candidate {

    pub fn new(
        p: &Problem,
        tour: &[u8],
        u: usize,
        k: usize
    )-> Self {
        let prev =
            if k == 0 {
                0
            } else {
                tour[k - 1] as usize
            };

        let next =
            if k == tour.len() {
                1
            } else {
                tour[k] as usize
            };

        let cost_increase =
              p.costs[(prev, u)] as f32
            + p.costs[(u, next)] as f32
            - p.costs[(prev, next)] as f32;

        let mut new_tour = tour.to_vec();
        new_tour.insert(k, u as u8);

        let status = p.eval_route(&new_tour);

        Candidate {
            u: u as u8,
            k,
            status,
            score_gain: p.scores[u] as f32,
            cost_increase,
        }
    }
}

            // let status = p.eval_route(&new_tour);



// type Picker = Box<dyn FnMut(Vec<Candidate>, &mut ThreadRng) -> Candidate>;
pub type Conversor = fn(&Candidate) -> f32;

// #[derive(Clone, Copy)]
// pub struct NamedStrategy {
//     pub name: &'static str,
//     pub cmp: StrategyFn,
// }

pub fn marginal_greedy(
    c: &Candidate,
) -> f32 {
    c.score_gain as f32
}

pub fn marginal_lazy(
    c: &Candidate,
) -> f32 {
    - (c.cost_increase as f32)
}

pub fn marginal_smart(
    c: &Candidate,
) -> f32 {
    c.score_gain as f32 / c.cost_increase
}

// Random Selection:
pub fn marginal_envy(
    c: &Candidate,
) -> f32 {
    (c.score_gain * c.score_gain) as f32 / c.cost_increase
}

pub fn total_envy(
    c: &Candidate,
) -> f32 {
    (c.status.total_score * c.status.total_score) as f32
        / c.status.total_consume
}

pub fn marginal_wise(
    c: &Candidate,
) -> f32 {
    c.score_gain as f32 / (c.cost_increase * c.cost_increase)
}

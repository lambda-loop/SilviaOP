

use crate::problem::TourStatus;
pub type Strategy = fn(TourStatus) -> f64;

const STRATEGYS: [Strategy;5] = [
    cost, score, greedy, envy, wise,
];

pub fn cost(ts: TourStatus) -> f64 {
    ts.cost as f64
}

pub fn score(ts: TourStatus) -> f64{
    ts.score as f64
}

pub fn greedy(ts: TourStatus) -> f64 {
    ts.score as f64 / ts.cost as f64
}

pub fn envy(ts: TourStatus) -> f64 {
    ts.score as f64 / (ts.cost as f64).sqrt()
}

pub fn wise(ts: TourStatus) -> f64 {
    (ts.score as f64).sqrt() / ts.cost as f64
}

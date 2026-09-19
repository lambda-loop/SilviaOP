//

use crate::problem::RouteStatus;
use std::cmp::Ordering;

pub type StrategyFn = fn(&RouteStatus, &RouteStatus) -> Ordering;
pub const ALL: &[StrategyFn] = &[greedy, lazy, smart, wise, envy];
pub const METHODS: [&str; 5] = ["greedy", "lazy", "smart", "wise", "envy"];

pub fn random(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    let pick = rand::random_range(0..5);
    let f = match pick {
        0 => greedy,
        1 => lazy,
        2 => smart,
        3 => wise,
        4 => envy,
        _ => panic!(),
    };

    f(l, r)
}

pub fn greedy(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    r.total_score.cmp(&l.total_score)
}

pub fn lazy(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    l.total_consume.total_cmp(&r.total_consume)
}

pub fn smart(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    let l = l.total_score as f32 / l.total_consume;
    let r = r.total_score as f32 / r.total_consume;
    r.total_cmp(&l)
}

pub fn wise(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    let l = l.total_score as f32 / l.total_consume.powi(2);
    let r = r.total_score as f32 / r.total_consume.powi(2);
    r.total_cmp(&l)
}

pub fn envy(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    let l = l.total_score.pow(2) as f32 / l.total_consume;
    let r = r.total_score.pow(2) as f32 / r.total_consume;
    r.total_cmp(&l)
}

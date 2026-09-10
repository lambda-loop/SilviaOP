//

use crate::problem::RouteStatus;
use std::cmp::Ordering;

pub type StrategyFn = fn(&RouteStatus, &RouteStatus) -> Ordering;
pub const ALL: &[StrategyFn] = &[greedy, lazy, smart, wise, envy];
pub const METHODS: [&str; 5] = ["greedy", "lazy", "smart", "wise", "envy"];

pub fn greedy(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    r.total_score.cmp(&l.total_score)
}

pub fn lazy(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    l.total_consume.total_cmp(&r.total_consume)
}

pub fn smart(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    let l = l.total_score as f32 / l.total_consume;
    let r = r.total_score as f32 / r.total_consume;
    l.total_cmp(&r)
}

pub fn wise(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    let l = l.total_score as f32 / l.total_consume.powi(2);
    let r = r.total_score as f32 / r.total_consume.powi(2);
    l.total_cmp(&r)
}

pub fn envy(l: &RouteStatus, r: &RouteStatus) -> Ordering {
    let l = l.total_score.pow(2) as f32 / l.total_consume;
    let r = r.total_score.pow(2) as f32 / r.total_consume;
    l.total_cmp(&r)
}

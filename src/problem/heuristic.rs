//

// simple heuristic: only cares about comparing between two by using at most
//  both the cost and the score, but no context yet

use crate::problem::Problem;
use std::cmp::Ordering;
use std::collections::HashSet as Set;

struct SimpleInfo {
    cost: f32,
    score: u16,
}

pub fn simple_heuristic(
    problem: &Problem,
    picker: fn(&SimpleInfo, &SimpleInfo) -> Ordering,
) -> Vec<u8> {
    let mut visited = Set::<u8>::new();
    let mut route = Vec::<u8>::new();

    let mut current_p = 0usize;
    let mut current_cost = 0.0;

    while let Some((p, SimpleInfo { cost: new_cost, .. })) = (2..problem.len)
        .filter(|&p| !visited.contains(&(p as u8)))
        .map(|p| {
            let cost = current_cost + problem.costs[(current_p, p)] + problem.costs[(p, 1)];
            let score = problem.scores[p];

            (p, SimpleInfo { cost, score })
        })
        .filter(|&(_, SimpleInfo { cost, .. })| cost <= problem.tmax)
        .max_by(|(_, l), (_, r)| picker(l, r))
    {
        current_cost = new_cost - problem.costs[(p, 1)];
        current_p = p;

        route.push(p as u8);
        visited.insert(p as u8);
    }

    route
}

pub fn apply_all(p: &Problem) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    out.push((String::from("greedy"), simple_heuristic(p, greedy)));
    out.push((String::from("lazy"), simple_heuristic(p, lazy)));
    out.push((String::from("smart"), simple_heuristic(p, smart)));
    out.push((String::from("wise"), simple_heuristic(p, wise)));
    out.push((String::from("envy"), simple_heuristic(p, envy)));
    out
}

pub fn greedy(
    SimpleInfo { score: score_l, .. }: &SimpleInfo,
    SimpleInfo { score: score_r, .. }: &SimpleInfo,
) -> Ordering {
    score_l.cmp(score_r)
}

pub fn lazy(
    SimpleInfo { cost: cost_l, .. }: &SimpleInfo,
    SimpleInfo { cost: cost_r, .. }: &SimpleInfo,
) -> Ordering {
    cost_r.total_cmp(cost_l)
}

pub fn smart(info_l: &SimpleInfo, info_r: &SimpleInfo) -> Ordering {
    let weight_l = info_l.score as f32 / info_l.cost;
    let weight_r = info_r.score as f32 / info_r.cost;
    weight_l.total_cmp(&weight_r)
}

pub fn wise(info_l: &SimpleInfo, info_r: &SimpleInfo) -> Ordering {
    let weight_l = info_l.score as f32 / info_l.cost * info_l.cost;
    let weight_r = info_r.score as f32 / info_r.cost * info_r.cost;
    weight_l.total_cmp(&weight_r)
}

pub fn envy(info_l: &SimpleInfo, info_r: &SimpleInfo) -> Ordering {
    let weight_l = (info_l.score * info_l.score) as f32 / info_l.cost;
    let weight_r = (info_r.score * info_r.score) as f32 / info_r.cost;
    weight_l.total_cmp(&weight_r)
}

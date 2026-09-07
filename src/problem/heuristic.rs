//

// simple heuristic: only cares about comparing between two by using at most
//  both the cost and the score, but no context yet

use crate::problem::{Problem, RouteStatus};
use std::cmp::Ordering;
use std::collections::HashSet as Set;

pub struct SimpleInfo {
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

pub fn since_you_already_best(problem: &Problem, mut r: Vec<u8>) -> Vec<u8> {
    // let r_len = r.len();
    let costs = &problem.costs;
    let mut unvisited = Set::<u8>::new();
    // FIX: its bad..
    for i in 2..problem.len {
        unvisited.insert(i as u8);
    }

    for &p in r.iter() {
        unvisited.remove(&p);
    }

    let tmax = problem.tmax;
    let RouteStatus {
        total_consume: mut cost,
        ..
    } = problem.eval_route(&r);

    loop {
        // let mut new_mid = None;
        let mut candidates = Vec::new();
        for i in 0..r.len() - 1 {
            let from = r[i] as usize;
            let to = r[i + 1] as usize;

            let opt_best = unvisited
                .iter()
                .cloned()
                .map(|mid_| {
                    let mid = mid_ as usize;
                    (
                        mid,
                        costs[(from, mid)] + costs[(mid, to)] - costs[(from, to)],
                    )
                })
                .filter(|(_, adjust)| cost + adjust < tmax)
                .min_by(|(_, a), (_, b)| a.total_cmp(b));
            if let Some((mid, cost)) = opt_best {
                candidates.push((i + 1, mid as u8, cost));
            }
        }

        if candidates.is_empty() {
            break;
        };

        let &(i, mid, adjust) = candidates
            .iter()
            .min_by(|(_, _, ca), (_, _, cb)| ca.total_cmp(cb))
            .unwrap();

        r.insert(i, mid);
        unvisited.remove(&mid);
        cost += adjust;
    }

    r
}
// just fills the already passing points cause why not?
// ure already there, u know?

pub fn since_you_already_fst(problem: &Problem, mut r: Vec<u8>) -> Vec<u8> {
    // let r_len = r.len();
    let costs = &problem.costs;
    let mut unvisited = Set::<u8>::new();
    // FIX: its bad..
    for i in 2..problem.len {
        unvisited.insert(i as u8);
    }

    for &p in r.iter() {
        unvisited.remove(&p);
    }

    let tmax = problem.tmax;
    let RouteStatus {
        total_consume: mut cost,
        ..
    } = problem.eval_route(&r);

    loop {
        let mut new_mid = None;
        'blk: for i in 0..r.len() - 1 {
            let from = r[i] as usize;
            let to = r[i + 1] as usize;

            for &mid_ in unvisited.iter() {
                let mid = mid_ as usize;
                let new_cost = cost + costs[(from, mid)] + costs[(mid, to)] - costs[(from, to)];
                if new_cost <= tmax {
                    new_mid = Some((i + 1, mid_, new_cost));
                    break 'blk;
                }
            }
        }

        if let Some((i, mid, new_cost)) = new_mid {
            r.insert(i, mid);
            unvisited.remove(&mid);
            cost = new_cost;
        } else {
            break;
        }
    }

    r
}

pub fn apply_all(p: &Problem) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    out.push((String::from("greedy"), simple_heuristic(p, greedy)));
    out.push((
        String::from("greedy-fst_fixed"),
        since_you_already_fst(p, simple_heuristic(p, greedy)),
    ));
    out.push((
        String::from("greedy-best-fixed"),
        since_you_already_best(p, simple_heuristic(p, greedy)),
    ));
    out.push((String::from("lazy"), simple_heuristic(p, lazy)));
    out.push((
        String::from("lazy-fst-fixed"),
        since_you_already_fst(p, simple_heuristic(p, lazy)),
    ));

    out.push((
        String::from("lazy-best-fixed"),
        since_you_already_best(p, simple_heuristic(p, lazy)),
    ));

    out.push((String::from("smart"), simple_heuristic(p, smart)));
    out.push((
        String::from("smart-fst-fixed"),
        since_you_already_fst(p, simple_heuristic(p, smart)),
    ));
    out.push((
        String::from("smart-best-fixed"),
        since_you_already_best(p, simple_heuristic(p, smart)),
    ));
    out.push((String::from("wise"), simple_heuristic(p, wise)));
    out.push((
        String::from("wise-fst-fixed"),
        since_you_already_fst(p, simple_heuristic(p, wise)),
    ));
    out.push((
        String::from("wise-best-fixed"),
        since_you_already_fst(p, simple_heuristic(p, wise)),
    ));
    out.push((String::from("envy"), simple_heuristic(p, envy)));
    out.push((
        String::from("envy-fst-fixed"),
        since_you_already_fst(p, simple_heuristic(p, envy)),
    ));
    out.push((
        String::from("envy-best-fixed"),
        since_you_already_fst(p, simple_heuristic(p, envy)),
    ));
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

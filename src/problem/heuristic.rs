//
pub mod experiment;
pub mod strategy;
use crate::problem::Problem;

use std::collections::HashSet;

use super::route::RouteStatus;
use std::cmp::Ordering;

pub fn single_best_insertion_by(
    p: &Problem,
    r: &mut Vec<u8>,
    unvisited: &mut HashSet<u8>,
    strategy: fn(&RouteStatus, &RouteStatus) -> Ordering,
) -> bool {
    let mut best: Option<(u8, usize, RouteStatus)> = None;

    let mut new_r = Vec::with_capacity(r.len() + 1);
    for k in 0..=r.len() {
        for &u in unvisited.iter() {
            new_r.clear();
            new_r.extend_from_slice(r);
            new_r.insert(k, u);

            let sts = p.eval_route(&new_r);

            if sts.total_consume > p.tmax {
                continue;
            }

            match &best {
                None => best = Some((u, k, sts)),
                Some((_, _, best_sts)) => {
                    if strategy(&sts, best_sts) == Ordering::Less {
                        best = Some((u, k, sts));
                    }
                }
            }
        }
    }

    match best {
        None => false,
        Some((u, k, _)) => {
            r.insert(k, u);
            unvisited.remove(&u);
            true
        }
    }
}

pub fn run_single(p: &Problem, s: fn(&RouteStatus, &RouteStatus) -> Ordering) -> Vec<u8> {
    let mut r = Vec::new();
    let mut unvisited = HashSet::new();
    for u in 2..p.len {
        unvisited.insert(u as u8);
    }

    while single_best_insertion_by(p, &mut r, &mut unvisited, s) {}
    r
}

use experiment::Result as ER;

pub fn run_single_experiment(
    p: &Problem,
    problem_name: String,
    s: fn(&RouteStatus, &RouteStatus) -> Ordering,
    method: String,
) -> ER {
    let mut r = Vec::new();
    let mut unvisited = HashSet::new();
    for u in 2..p.len {
        unvisited.insert(u as u8);
    }

    while single_best_insertion_by(p, &mut r, &mut unvisited, s) {}
    let sts = p.eval_route(&r);
    ER {
        problem_name,
        method,
        cost: sts.total_consume,
        score: sts.total_score,
        route: r,
    }
}

pub fn run_multi_experiment(
    ps: &[Problem],
    p_names: &[String],
    s: fn(&RouteStatus, &RouteStatus) -> Ordering,
    method: String,
) -> Vec<ER> {
    let mut r = Vec::with_capacity(100);
    let mut unvisited = HashSet::new();

    let mut ers = Vec::new();
    for (p, problem_name) in ps.iter().zip(p_names.iter()) {
        r.clear();
        for u in 2..p.len {
            unvisited.insert(u as u8);
        }

        while single_best_insertion_by(p, &mut r, &mut unvisited, s) {}
        let sts = p.eval_route(&r);
        let er = ER {
            problem_name: problem_name.clone(),
            method: method.clone(),
            cost: sts.total_consume,
            score: sts.total_score,
            route: r.clone(),
        };
        ers.push(er);
    }

    ers
}

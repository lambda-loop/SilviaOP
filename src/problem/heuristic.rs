//
pub mod dispersion;
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
        unvisited.clear();
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

// with single valued strategies
pub fn one_experiment(p: &Problem, p_name: &str) -> Vec<ER> {
    let mut exp_ress = Vec::new();
    for (&f, &m) in strategy::ALL.iter().zip(strategy::METHODS.iter()) {
        for _ in 0..NTIMES {
            let exp_res = run_single_experiment(p, p_name.to_string(), f, m.to_string());
            exp_ress.push(exp_res);
        }
    }

    for _ in 0..NTIMES {
        let exp_res = run_single_experiment(
            p,
            p_name.to_string(),
            strategy::random,
            "random".to_string(),
        );
        exp_ress.push(exp_res);
    }

    exp_ress
}
type Strategy = fn(&RouteStatus, &RouteStatus) -> Ordering;

const NTIMES: usize = 100_000;
pub fn single_results(
    p: &Problem,
    p_name: &str,
    s: Strategy,
    s_name: &str,
) -> Vec<EResult> {
    let mut scores = Vec::with_capacity(NTIMES);

    for _ in 0..NTIMES {
        let mut r = Vec::with_capacity(p.len);

        let mut unvisited = HashSet::new();
        for u in 2..p.len {
            unvisited.insert(u as u8);
        }

        while single_best_insertion_by(p, &mut r, &mut unvisited, s) {}

        let RouteStatus { total_score, .. } = p.eval_route(&r);
        scores.push(total_score);
    }

    scores.sort_unstable();

    let n = scores.len();

    let worst = scores[0] as f64;
    let best = scores[n - 1] as f64;

    let median = if n % 2 == 0 {
        (scores[n / 2 - 1] as f64 + scores[n / 2] as f64) / 2.0
    } else {
        scores[n / 2] as f64
    };

    let q1 = scores[n / 4] as f64;
    let q3 = scores[3 * n / 4] as f64;

    let average =
        scores.iter().map(|&x| x as f64).sum::<f64>() / n as f64;

    [
        (InfoType::Median, median),
        (InfoType::Q1, q1),
        (InfoType::Q3, q3),
        (InfoType::Best, best),
        (InfoType::Worst, worst),
        (InfoType::Average, average),
    ]
    .into_iter()
    .map(|(info_t, info)| EResult {
        p_name: p_name.to_string(),
        s_name: s_name.to_string(),
        info_t,
        info,
    })
    .collect()
}

pub fn multi_results(p: &Problem, p_name: &str) -> Vec<EResult> {
    let mut exp_ress = Vec::new();

    for (&f, &m) in strategy::ALL.iter().zip(strategy::METHODS.iter()) {
        exp_ress.extend(single_results(p, p_name, f, m));
    }

    exp_ress.extend(single_results(
        p,
        p_name,
        strategy::random,
        "random",
    ));

    println!("done");

    exp_ress
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoType {
    Median,
    Q1,
    Q3,
    Best,
    Worst,
    Average,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EResult {
    pub p_name: String,
    pub s_name: String,
    pub info_t: InfoType,
    pub info: f64,
}

impl EResult {
    pub fn to_csv(&self) -> String {
        format!("{},{},{:?},{}",
                self.p_name,
                self.s_name,
                self.info_t,
                self.info,
        )
    }

    pub fn header() -> String {
        format!("p_name,s_name,info_type,info")
    }
}

//

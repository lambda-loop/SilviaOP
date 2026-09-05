//

pub mod heuristic;
mod matrix;
mod pos;
pub mod route;

use matrix::SquareMatrix;
use pos::Node as Pos2D;
use route::*;

#[derive(Debug)]
pub struct Problem {
    pub tmax: f32,
    pub costs: SquareMatrix<f32>,
    pub scores: Vec<u16>,
    pub len: usize, // num_points
}

impl Problem {
    pub fn new(raw_input: &str) -> Self {
        //
        let mut lines = raw_input.lines();
        let mut fst_line = lines.next().unwrap().trim().split_whitespace();

        let tmax: f32 = fst_line.next().unwrap().parse().unwrap();

        let mut points = Vec::new();
        for line in lines {
            let point = Pos2D::parse(line);
            points.push(point);
        }

        let len = points.len();
        let scores = points.iter().map(|p| p.score).collect();

        let mut costs = SquareMatrix::<f32>::new(len);

        for i in 0..len {
            for j in 0..len {
                costs[(i, j)] = points[i].euc_dist(&points[j]);
            }
        }

        Problem {
            tmax,
            costs,
            scores,
            len,
        }
    }

    pub fn eval_route(&self, r: &[u8]) -> RouteStatus {
        let r_len = r.len();
        let mut total_score: u16 = r.iter().map(|&p| self.scores[p as usize]).sum();
        let mut total_consume = 0.;

        total_consume += self.costs[(0, r[0] as usize)];
        for i in 0..r_len - 1 {
            let from = r[i] as usize;
            let to = r[i + 1] as usize;

            total_consume += self.costs[(from, to)];
        }

        total_consume += self.costs[(r[r_len - 1] as usize, 1)];

        RouteStatus {
            total_score,
            total_consume,
        }
    }
}

//TODO: bitset when?
use std::collections::HashSet as Set;
pub fn greedy(problem: &Problem) -> Vec<u8> {
    let mut visited = Set::<u8>::new();
    let mut route = Vec::<u8>::new();

    let mut current_p = 0usize;
    let mut current_cost = 0.0;

    while let Some((p, new_cost)) = (2..problem.len)
        .filter(|&p| !visited.contains(&(p as u8)))
        .map(|p| {
            let cost = current_cost + problem.costs[(current_p, p)] + problem.costs[(p, 1)];

            (p, cost)
        })
        .filter(|&(_, cost)| cost <= problem.tmax)
        .max_by_key(|&(p, _)| problem.scores[p])
    {
        current_cost = new_cost - problem.costs[(p, 1)];
        current_p = p;

        route.push(p as u8);
        visited.insert(p as u8);
    }

    route
}

//

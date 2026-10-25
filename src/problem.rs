//

mod op_lib;
mod classical_lib;
mod pos;
pub mod tour;
// pub mod route;

// use matrix::SquareMatrix;
use grid::Grid;
pub use tour::*;

// pub static mut EVALS: usize = 0;

#[derive(Debug)]
pub struct Problem {
    pub tmax  : f64      ,
    pub costs : Grid<f64>,
    pub scores: Vec<u16> ,
    pub len   : usize    , // num_points
}

impl Problem {

    // classical
    pub fn from_classical(input: &str) -> Self {
        classical_lib::parse(input)
    }

    // op lib
    pub fn from_op(input: &str) -> Self {
        op_lib::parse(input)
    }

    // just assumes thats everything ok with the tour.
    pub fn eval_tour(
        &self,
        used: &mut u64,
        budget: u64,
        tour: &[Point]
    ) -> Option<TourStatus> {
        if *used >= budget { return None };
        *used += 1;
        let rs = self.eval_route(tour);
        Some (
            TourStatus {
                cost: rs.total_consume,
                score: rs.total_score
            }
        )
    }

    fn eval_route(&self, r: &[Point]) -> RouteStatus {
        if r.is_empty() {
            return RouteStatus {
                total_consume: self.costs[(0, 1)],
                total_score: 0,
            };
        }        let r_len = r.len();
        let mut total_score: u32 = r
            .iter()
            .map(|&p| self.scores[p as usize] as u32)
            .sum();

        let mut total_consume = 0.;

        total_consume += self.costs[(0, r[0] as usize)];
        for i in 0..r_len - 1 {
            let from = r[i] as usize;
            let to = r[i + 1] as usize;

            total_consume += self.costs[(from, to)];
        }

        total_consume += self.costs[(r[r_len - 1] as usize, 1)];

        // unsafe {  EVALS += 1; };
        RouteStatus {
            total_score: total_score as u32,
            total_consume,
        }

    }
}

//TODO: bitset when?
use std::collections::HashSet as Set;
pub fn greedy(problem: &Problem) -> Tour {
    let p_len = problem.len;
    let mut visited = Set::<Point>::new();
    let mut tour = Tour::new(p_len);

    let mut current_p = 0usize;
    let mut current_cost = 0.0;

    while let Some((p, new_cost)) = (2..p_len)
        .filter(|&p| !visited.contains(&(p as Point)))
        .map(|p| {
            let cost = current_cost + problem.costs[(current_p, p)] + problem.costs[(p, 1)];

            (p, cost)
        })
        .filter(|&(_, cost)| cost <= problem.tmax)
        .max_by_key(|&(p, _)| problem.scores[p])
    {
        current_cost = new_cost - problem.costs[(p, 1)];
        current_p = p;

        tour.push(p as Point);
        visited.insert(p as Point);
    }

    tour
}

pub struct OutOfCredits;

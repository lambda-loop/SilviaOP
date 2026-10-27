
pub mod two_opt;

use crate::Tour;
use crate::problem::Point;

use rand::{*, rngs::SmallRng};
use grid::Grid;

// A simple and incremental implementation
// 1 -> ILS + 2-opt	
// 2 -> ILS + 2-opt + relocate	
// 3 -> ILS + 2-opt + Or-opt	

pub fn ils(
    tour : &mut Tour,
    rng  : &mut SmallRng,
    costs: &Grid<f64>,
    max_iters: u64 // max iters has to be greater than 10.
) -> f64 {
    let mut current_tour = tour.as_slice().to_owned();
    let mut current_cost = {
        let mut cost     = 0.;
        let mut previous = 0usize;

        for &point in tour.iter() {
            cost += costs[(previous, point as usize)];
            previous = point as usize;
        }

        cost + costs[(previous, 1)]
        
    };
    let mut best_cost = current_cost;

    const D: u64 = 10;
    let check = max_iters / D;
    for round in 0..D {
        let new_cost = two_opt::two_opt ( 
            &mut current_tour,
            current_cost,
            rng, costs,
            check,
        );
        
        if new_cost != current_cost {
            if new_cost < best_cost {
                tour.as_mut_slice()
                    .copy_from_slice(&current_tour);
                best_cost = new_cost;

            }
            current_cost = new_cost;

        } else if round + 1 < D {
            current_cost = double_bridge(
                &mut current_tour, rng, costs,
            );
        }
    }

    best_cost
}

// disturbance (AI GENERATED)
pub fn double_bridge(
    tour: &mut [Point],
    rng: &mut SmallRng,
    costs: &Grid<f64>,
) -> f64 {
    let n = tour.len();

    if n >= 3 {
        let mut cuts = [0usize; 4];

        for k in 0..cuts.len() {
            loop {
                let cut = rng.random_range(0..=n);

                if !cuts[..k].contains(&cut) {
                    cuts[k] = cut;
                    break;
                }
            }
        }

        cuts.sort_unstable();
        let [a, b, c, d] = cuts;

        tour[a..b].reverse();
        tour[b..c].reverse();
        tour[c..d].reverse();
        tour[a..d].reverse();
    }

    let mut total_cost = 0.0;
    let mut previous = 0usize;

    for &point in tour.iter() {
        let point = point as usize;
        total_cost += costs[(previous, point)];
        previous = point;
    }

    total_cost + costs[(previous, 1)]
}

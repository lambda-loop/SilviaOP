use crate::problem::tour::Point;

use grid::Grid;

// Kinda ai..
pub fn heuristic(
    tour: &mut [Point],
    costs: &Grid<f64>,
) {
    let mut last = 0usize;

    for position in 0..tour.len() {
        let mut best = position;
        let mut best_cost = costs[(last, tour[position] as usize)];

        for candidate in position + 1..tour.len() {
            let cost = costs[(last, tour[candidate] as usize)];

            if cost < best_cost {
                best = candidate;
                best_cost = cost;
            }
        }

        tour.swap(position, best);
        last = tour[position] as usize;
    }
}

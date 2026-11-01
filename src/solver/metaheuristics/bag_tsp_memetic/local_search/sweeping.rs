

use grid::Grid;
use crate::{
    Problem, problem::{
        tour::*,
        TourStatus, 
    },
};

use super::Fitness;

pub fn sweeping(
    tour : &mut [Point],
    costs: &Grid<f64>,
) -> bool {
    for r in (0..tour.len().saturating_sub(1)).step_by(2) {
        let s = r + 1;

        let (a, b, c, d) = (
            if r == 0 { 0 } else { tour[r - 1] as usize },
            tour[r] as usize,
            tour[s] as usize,
            if s + 1 == tour.len() { 1 } else { tour[s + 1] as usize },
        );

        let old_cost =
              costs[(a, b)]
            + costs[(b, c)]
            + costs[(c, d)];

        let new_cost =
              costs[(a, c)]
            + costs[(c, b)]
            + costs[(b, d)];

        if new_cost < old_cost {
            tour.swap(r, s);
            return true;
        }
    }

    false
}

// AI Generated
fn compute_two_swap(
    tour : &[Point],
    costs: &Grid<f64>,
    r    : usize,
    s    : usize,
) -> f64 {
    if tour.is_empty() {
        return costs[(0, 1)];
    }

    assert!(r < tour.len() && s < tour.len());

    let (r, s) = if r <= s { (r, s) } else { (s, r) };

    if r == s {
        return compute_cost(tour.iter(), costs);
    }

    let points = tour[..r].iter()
        .chain(std::iter::once(&tour[s]))
        .chain(tour[r + 1..s].iter())
        .chain(std::iter::once(&tour[r]))
        .chain(tour[s + 1..].iter());

    compute_cost(points, costs)
}

// AI Generated
#[inline]
fn compute_cost<'a>(
    points: impl Iterator<Item = &'a Point>,
    costs : &Grid<f64>,
) -> f64 {
    let mut cost = 0.;
    let mut previous = 0usize;

    for &point in points {
        cost += costs[(previous, point as usize)];
        previous = point as usize;
    }

    cost + costs[(previous, 1)]
}

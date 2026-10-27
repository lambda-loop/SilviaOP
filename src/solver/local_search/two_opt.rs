
use rand::{
    Rng, *,
};


use grid::Grid;
use crate::{
    problem::OutOfCredits,
    problem::{Point},
};

use super::LocalSearchCtx;

// notice that, by doing k-opts enchanges, we never go for the score..
// so we acctually CANT spend any credit at all..
pub fn two_opt_until (
    // it keep changing until it spend all the iters..
    ctx: &mut LocalSearchCtx<'_>,
) -> Result<(), OutOfCredits> {
    let tour_len = ctx.tour.len();

    let iters = ctx.iters;
    if tour_len < 2 || iters == 0 {
        return Ok(());
    }

    let costs         = &ctx.p.costs;
    let mut best_cost = ctx.cost;

    for _ in 0..iters {

        let begin = ctx.rng.random_range(0..tour_len - 1);
        let end   = ctx.rng.random_range(begin + 1..tour_len);

        let new_cost = eval_two_opted_cost(
            ctx.tour, costs,
            begin as usize,
            end   as usize,
        );

        if new_cost < best_cost {
            best_cost = new_cost;
            ctx.tour.reverse_in(begin as usize, end as usize);
        }

    }

    ctx.cost = best_cost;
    Ok(())
}

fn eval_two_opted_cost(
    tour : &[Point],
    costs: &Grid<f64>,
    begin: usize,
    end  : usize,
) -> f64 {
    if tour.is_empty() {
        return costs[(0, 1)];
    }

    let points = tour[..begin].iter()
        .chain(tour[begin..=end].iter().rev())
        .chain(tour[end + 1..].iter());

    let mut cost     = 0.;
    let mut previous = 0usize;

    for &point in points {
        cost += costs[(previous, point as usize)];
        previous = point as usize;
    }

    cost + costs[(previous, 1)]
}




use rand::{
    Rng, *,
};

use crate::{
    OutOfCredits,
    problem::{Point, route::TourStatus},
};

use super::LocalSearchCtx;

pub fn two_opt_fst_improvement(
    ctx: &mut LocalSearchCtx<'_>,
) -> Result<(), OutOfCredits> {
    let tour_len = ctx.tour.len();

    if tour_len < 2 || ctx.neighborhood == 0 {
        return Ok(());
    }

    let problem = ctx.p;
    let current_status = *ctx.status;
    let current_fitness = *ctx.fit;

    let edge_cost = |from_point: Point, to_point: Point| {
        problem.costs[(from_point as usize, to_point as usize)]
    };

    for _ in 0..ctx.neighborhood {
        if *ctx.used >= ctx.budget {
            return Err(OutOfCredits);
        }
        *ctx.used += 1;

        let start_index = ctx.rng.random_range(0..tour_len - 1);
        let end_index = ctx.rng.random_range(start_index + 1..tour_len);

        let (start_index, end_index) =
            (start_index as usize, end_index as usize);

        let tour = ctx.tour.as_slice();

        let previous_point = if start_index == 0 {
            0
        } else {
            tour[start_index - 1]
        };

        let first_point = tour[start_index];
        let last_point = tour[end_index];

        let next_point = if end_index + 1 == tour.len() {
            1
        } else {
            tour[end_index + 1]
        };

        let mut cost_delta =
            edge_cost(previous_point, last_point)
            + edge_cost(first_point, next_point)
            - edge_cost(previous_point, first_point)
            - edge_cost(last_point, next_point);

        for edge in tour[start_index..=end_index].windows(2) {
            cost_delta +=
                edge_cost(edge[1], edge[0])
                - edge_cost(edge[0], edge[1]);
        }

        let candidate_status = TourStatus {
            cost: current_status.cost + cost_delta,
            ..current_status
        };

        let candidate_fitness = (ctx.fitness)(candidate_status);

        if candidate_fitness > current_fitness
            || (candidate_fitness == current_fitness
                && candidate_status.cost < current_status.cost)
        {
            ctx.tour[start_index..=end_index].reverse();
            *ctx.status = candidate_status;
            *ctx.fit = candidate_fitness;

            return Ok(());
        }
    }

    Ok(())
}

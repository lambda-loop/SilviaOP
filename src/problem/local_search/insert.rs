
use rand::{
    Rng, *,
};

use crate::problem::matrix::SquareMatrix;
use crate::{
    OutOfCredits,
    problem::{Point, route::TourStatus},
};

use super::LocalSearchCtx;

// only feasible and acctually use a random strategy..
pub fn insert<'a>(
    mut ctx: &mut LocalSearchCtx<'a>
) -> Result<(Vec<u8>, TourStatus, f32), OutOfCredits> {
    if *ctx.used >= ctx.budget { return Err(OutOfCredits) }
    let bound = ctx.visited.count_zeroes(2..ctx.p.len);
    if bound == 0 || ctx.neighborhood == 0 {
        return Ok((ctx.tour.clone(), ctx.status, ctx.fit));
    }

    let (
        mut best_tour,
        mut best_status,
        mut best_fitness,
    ) = {
        let idx   = ctx.rng.random_range(0..bound);
        let mut count = 0;
        let mut zeroes = ctx.visited.zeroes();
        let v = 'l: loop {
            let v = zeroes.next().unwrap();
            if count == idx { break 'l v }
            count += 1;
        };

        let idx = ctx.rng.random_range(0..=ctx.tour.len());
        let mut tour = ctx.tour.clone();
        tour.insert(idx, v as u8);

        let Some(status) = ctx.p.eval_tour(
            ctx.used,
            ctx.budget,
            &tour
        ) else { return Err(OutOfCredits) };
        let fitness = (ctx.fitness)(status);

        (tour, status, fitness)
    };

    for _ in 1..ctx.neighborhood {
        let idx   = ctx.rng.random_range(0..bound);
        let mut count = 0;
        let mut zeroes = ctx.visited.zeroes();
        let v = 'l: loop {
            let v = zeroes.next().unwrap();
            if count == idx { break 'l v }
            count += 1;
        };

        let idx = ctx.rng.random_range(0..=ctx.tour.len());
        let mut tour = ctx.tour.clone();
        tour.insert(idx, v as u8);

        let Some(status) = ctx.p.eval_tour(
            ctx.used,
            ctx.budget,
            &tour
        ) else { return Ok((best_tour, best_status, best_fitness)) };
        let fitness = (ctx.fitness)(status);

        if fitness > best_fitness {
            best_tour = tour;
            best_status = status;
            best_fitness = fitness;
            
        }
    }

    Ok((best_tour, best_status, best_fitness))
}

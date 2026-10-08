
use rand::*;
use rand::rngs::ThreadRng;

use fixedbitset::FixedBitSet as BitSet;

struct LocalSearchCtx<'a> {
    p            : &'a Problem          ,
    tour         : Vec<u8>              ,
    used         : &'a mut usize        ,
    budget       : usize                ,
    neighborhood : usize, // amount of tours to generate
    rng          : &'a mut ThreadRng    ,
    fitness      : Box<dyn Fn(&[u8]) -> f32 + 'a>,
    visited      : BitSet
}

// cria N e depois pega a mlr. Nao mantem a original!

use crate::consume_credit;
use crate::{problem::Problem, OutOfCredits};
pub fn two_opt<'a>(
    ctx: &mut LocalSearchCtx<'a>
) -> Result<Vec<u8>, OutOfCredits> {
    let n = ctx.tour.len();

    if n < 2 || ctx.neighborhood == 0 {
        return Ok(ctx.tour.clone());
    }

    let (mut best_tour, mut best_fitness) = {
        let i = ctx.rng.random_range(0..n-1);
        let j = ctx.rng.random_range(i+1..n);
        let mut tour = ctx.tour.clone();
        tour[i..=j].reverse();

        consume_credit!(ctx.used, ctx.budget);
        let fitness = (ctx.fitness)(&tour);

        (tour, fitness)
    };

    for _ in 1..ctx.neighborhood {
        let i = ctx.rng.random_range(0..n-1);
        let j = ctx.rng.random_range(i+1..n);

        let mut tour = ctx.tour.clone();
        tour[i..=j].reverse();

        consume_credit!(ctx.used, ctx.budget);
        let fitness = (ctx.fitness)(&tour);

        if fitness > best_fitness {
            best_tour = tour;
            best_fitness = fitness;
        }
    }

    Ok(best_tour)
}

pub fn insert<'a>(ctx: &mut LocalSearchCtx<'a>) -> Result<Vec<u8>, OutOfCredits> {
    let bound = ctx.visited.count_zeroes(2..ctx.p.len);
    if bound == 0 || ctx.neighborhood == 0 {
        return Ok(ctx.tour.clone());
    }

    let (mut best_tour, mut best_fitness) = {
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

        consume_credit!(ctx.used, ctx.budget);
        let fitness = (ctx.fitness)(&tour);
        (tour, fitness)
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

        consume_credit!(ctx.used, ctx.budget);
        let fitness = (ctx.fitness)(&tour);

        if fitness > best_fitness {
            best_tour = tour;
            best_fitness = fitness;
            
        }
    }

    Ok(best_tour)
}

pub fn remove<'a>(ctx: &mut LocalSearchCtx<'a>) -> Result<Vec<u8>, OutOfCredits> {
    let tour_len = ctx.tour.len();
    if tour_len < 1 || ctx.neighborhood == 0 {
        return Ok(ctx.tour.clone());
    }

    let (mut best_tour, mut best_fitness) = {
        let mut tour = ctx.tour.to_vec();
        let idx = ctx.rng.random_range(0..tour_len);
        tour.remove(idx);

        consume_credit!(ctx.used, ctx.budget);
        let fitness = (ctx.fitness)(&tour);
        (tour, fitness)
    };

    for _ in 1..ctx.neighborhood {
        let mut tour = ctx.tour.to_vec();
        let idx = ctx.rng.random_range(0..tour_len);
        tour.remove(idx);

        consume_credit!(ctx.used, ctx.budget);
        let fitness = (ctx.fitness)(&tour);
        if fitness > best_fitness {
            best_tour    = tour;
            best_fitness = fitness;
        }
    }

    Ok(best_tour)
}

// 2-opt	Reorganiza a rota para reduzir seu custo, preservando os vértices visitados.
// Inserção	Adiciona vértices externos, priorizando a maior razão prêmio / aumento de custo.
// Remoção	Recupera a viabilidade de rotas acima do orçamento, priorizando a menor razão prêmio / economia de custo.


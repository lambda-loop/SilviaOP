
use rand::*;
use rand::rngs::ThreadRng;

use fixedbitset::FixedBitSet as BitSet;
use crate::{
    OutOfCredits,
    problem::{
        TourStatus,
        Problem,
    },
};

pub type LocalSearcher =
    Box<dyn Fn(&mut LocalSearchCtx)
               -> Result<(Vec<u8>, TourStatus, f32), OutOfCredits>>;
pub struct LocalSearchCtx<'a> {
    pub p            : &'a Problem          ,
    pub tour         : Vec<u8>              ,
    pub status       : TourStatus,
    pub fit          : f32,
    pub used         : &'a mut u64,
    pub budget       : u64,
    pub neighborhood : usize, // amount of tours to generate
    pub rng          : &'a mut ThreadRng    ,
    pub fitness      : Box<dyn Fn(TourStatus) -> f32 + 'a>,
    pub visited      : BitSet
}

// cria N e depois pega a mlr. Nao mantem a original!

pub fn two_opt<'a>(
    ctx: &mut LocalSearchCtx<'a>
) -> Result<(Vec<u8>, TourStatus, f32), OutOfCredits> {
    if *ctx.used >= ctx.budget { return Err(OutOfCredits) }
    let n = ctx.tour.len();

    if n < 2 || ctx.neighborhood == 0 {
        return Ok((ctx.tour.clone(), ctx.status, ctx.fit));
    }

    // generates first tour
    let (
        mut best_tour,
        mut best_status,
        mut best_fitness,
    ) = {
        let i = ctx.rng.random_range(0..n-1);
        let j = ctx.rng.random_range(i+1..n);
        let mut tour = ctx.tour.clone();
        tour[i..=j].reverse();

        let Some(status) = ctx.p.eval_tour(
            ctx.used,
            ctx.budget,
            &tour
        ) else { return Err(OutOfCredits) };
        let fitness = (ctx.fitness)(status);

        (tour, status, fitness)
    };

    for _ in 1..ctx.neighborhood {
        let i = ctx.rng.random_range(0..n-1);
        let j = ctx.rng.random_range(i+1..n);

        let mut tour = ctx.tour.clone();
        tour[i..=j].reverse();

        let Some(status) = ctx.p.eval_tour(
            ctx.used,
            ctx.budget,
            &tour
        ) else {
            return Ok((best_tour, best_status, best_fitness))
        };
        let fitness = (ctx.fitness)(status);
        if fitness > best_fitness {
            best_tour = tour;
            best_status = status;
            best_fitness = fitness;
        }
    }

    Ok((best_tour, best_status, best_fitness))
}

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

pub fn remove<'a>(
    ctx: &mut LocalSearchCtx<'a>
) -> Result<(Vec<u8>, TourStatus, f32), OutOfCredits> {
    if *ctx.used >= ctx.budget { return Err(OutOfCredits) }
    let tour_len = ctx.tour.len();
    if tour_len < 1 || ctx.neighborhood == 0 {
        return Ok((ctx.tour.clone(), ctx.status, ctx.fit));
    }

    let (
        mut best_tour,
        mut best_status, 
        mut best_fitness,
    ) = {
        let mut tour = ctx.tour.to_vec();
        let idx = ctx.rng.random_range(0..tour_len);
        tour.remove(idx);

        let Some(status) = ctx.p.eval_tour(
            ctx.used,
            ctx.budget,
            &tour
        ) else { return Err(OutOfCredits) };
        let fitness = (ctx.fitness)(status);
        (tour, status, fitness)
    };

    for _ in 1..ctx.neighborhood {
        let mut tour = ctx.tour.to_vec();
        let idx = ctx.rng.random_range(0..tour_len);
        tour.remove(idx);

        let Some(status) = ctx.p.eval_tour(
            ctx.used,
            ctx.budget,
            &tour
        ) else { return Ok((best_tour, best_status, best_fitness)) };
        let fitness = (ctx.fitness)(status);
        if fitness > best_fitness {
            best_tour    = tour;
            best_status  = status;
            best_fitness = fitness;
        }
    }

    Ok((best_tour, best_status, best_fitness))
}

// thats used after some mutation!
pub fn local_search<'a>(
    ctx: &mut LocalSearchCtx<'a>
) -> Result<(Vec<u8>, TourStatus, f32), OutOfCredits> {
    search(Box::new(two_opt), ctx);
    search(Box::new(insert ), ctx);
    search(Box::new(two_opt), ctx);
    search(Box::new(remove ), ctx);
    search(Box::new(two_opt), ctx);
    Ok((ctx.tour.to_owned(), ctx.status, ctx.fit))
}

use super::update_visited;
pub fn search<'a>(
    searcher: LocalSearcher,
    ctx: &mut LocalSearchCtx,
) {
    let (tour, status, fit) = match (searcher)(ctx) {
        Err(_) => return,
        Ok(new) => new,
    };
        
    ctx.tour = tour;
    ctx.status = status;
    ctx.fit = fit;
    update_visited(&ctx.tour, &mut ctx.visited);
}


// 2-opt	Reorganiza a rota para reduzir seu custo, preservando os vértices visitados.
// Inserção	Adiciona vértices externos, priorizando a maior razão prêmio / aumento de custo.
// Remoção	Recupera a viabilidade de rotas acima do orçamento, priorizando a menor razão prêmio / economia de custo.


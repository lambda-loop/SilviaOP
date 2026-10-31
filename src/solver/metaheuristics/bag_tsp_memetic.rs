
use std::mem::MaybeUninit;
use fixedbitset::FixedBitSet as BitSet;
use grid::Grid;
use rand::{
    *,
    rngs::SmallRng,
};


mod cross       ; use cross::*  ;
mod genesis     ; use genesis::*;
mod mutation    ; use mutation::*;
mod local_search; use local_search::*;

use crate::{
    Problem, problem::{
        tour::*,
        TourStatus, 
    },
};

type Fitness    = fn(TourStatus) -> f64;
type Assembly   = fn(
    &mut Tour, &mut SmallRng, 
    costs: &Grid<f64>,
    max_iters: u64
);


pub struct MemeticBagTsp {
    problem   : Problem   ,
    budget    : u64       ,
    genesis   : Genesis   ,
    pop_size  : usize     ,
    elite_size: usize     ,
    round_len : usize     , 
    fitness   : Fitness   ,
    rng       : SmallRng  ,
    crosser   : Crossing  , 
    assembler : Assembly  ,
    assembler_iters : u64 ,
    mutator   : Mutation  ,
    searcher : LocalSearch, 
}

impl MemeticBagTsp {
    fn run (self) -> Tour {
        let mut used    = 0               ;
        let budget      = self.budget     ;
        let mut rng     = self.rng        ;
        let p           = &self.problem   ;
        let tmax        = p.tmax          ;
        let costs       = &p.costs        ;
        let problem_len = self.problem.len;
        let elite_size  = self.elite_size ;
        let round_len   = self.round_len  ;
        let pop_size    = self.pop_size   ;

        let aiters = self.assembler_iters;
        let mut chairs: Box<_> = Vec::with_capacity(elite_size)
            .into_boxed_slice();

        let mut pop: Box<_> = (self.genesis) (
            problem_len,
            &mut rng,
            pop_size,
        ).into_iter()
         .map(|tour| {
             let ts = p.eval_tour(
                 &mut used,
                 budget,
                 tour.as_slice()
             ).expect(
                 "credits were not enough for this tsp_memetic run!"
             );

             let fit = (self.fitness) (ts);
             (tour, fit, ts)
         }).collect();

        let mut occupied   = BitSet::with_capacity(problem_len);
        let mut best_tour  = Tour::new(problem_len);
        let mut best_score = 0;

        while used < budget {
            tournament(&mut rng, round_len, &pop, &mut chairs);
            if let Some((tour, _, ts)) = extracts_best_feasible(
                &pop, best_score, tmax,
            ) && ts.score > best_score {
                best_tour = tour.clone();
                best_score = ts.score;
            }

            for (t, _, _) in &pop {
                occupied.union_with(t.visited());
            }

            for (t, f, ts) in pop.iter_mut() {
                let l = rng.random_range(0..elite_size);
                let r = rng.random_range(0..elite_size);

                let l = chairs[l].visited_block();
                let r = chairs[r].visited_block();
                let child = t.visited_block_mut();

                (self.crosser)(l, r, child, &mut rng);
                t.fix();

                (self.assembler)(
                    t, &mut rng,
                    costs, aiters
                );

                (self.mutator) (t, &mut rng, &occupied, costs, tmax);
                *ts = (self.searcher) (t, &mut used, budget, p);
                if used >= budget { return best_tour }
                *f  = (self.fitness) (*ts);
            }

            occupied.clear();
        }

        best_tour
    }
}

// we dont know anymoer if its sorted or not
fn extracts_best_feasible(
    // tour, fit, coust
    pop: &[(Tour, f64, TourStatus)],
    best_score: u32, 
    tmax: f64,
) -> Option<&(Tour, f64, TourStatus)> {
    let mut best_found = None;
    let mut max_score  = best_score;

    for p@(_, _, ts) in pop {
        // short circuit when
        if ts.cost <= tmax && ts.score > max_score {
            max_score = ts.score;
            best_found = Some(p);
        }
    }

    best_found
}

fn tournament (
    rng      : &mut SmallRng             ,
    round_len: usize                     ,
    pop      : &[(Tour, f64, TourStatus)],
    chairs   : &mut [Tour]               ,
) {

    todo!()
}


fn round<'a> (
    rng: &mut SmallRng,
    round_len: usize,
    pop: &'a[(Tour, f64, TourStatus)],
) -> &'a Tour {

    todo!()
}

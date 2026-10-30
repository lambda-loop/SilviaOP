
mod cross;

use std::mem::MaybeUninit;
use fixedbitset::FixedBitSet as BitSet;
use grid::Grid;
use rand::{
    *,
    rngs::SmallRng,
};

use crate::{
    Problem, Tour, problem::Point,
};

type Genesis  = fn(problem_size: usize, pop_size: usize) -> Box<[Tour]>;
type Crossing = fn(
    l    : &[usize], // the bytes 
    r    : &[usize], // the byts
    child: &mut [usize],
    rng  : &mut SmallRng, 
);

type Renovation = fn(&Box<Tour>, SmallRng);
type Fitness    = fn(&Problem, &[Point]) -> f64;
type Assembly   = fn(
    &mut Tour, &mut SmallRng, 
    costs: &Grid<f64>,
    max_iters: u64
);

type Mutation   = fn(
    tour : &mut Tour,
    rng  : &mut SmallRng,
    used : &BitSet,
    costs: &Grid<f64>,
    tmax : f64,
); 

// intense
type LocalSearch = fn() -> f64;

pub struct MemeticBagTsp {
    problem   : Problem   ,
    budget    : u64       ,
    genesis   : Genesis   ,
    pop_size  : usize     ,
    elite_size: usize     ,
    // royal_size: usize     ,
    // renovator: Renovation,
    fitness   : Fitness   ,
    rng       : SmallRng  ,
    crosser   : Crossing  , 
    assembler : Assembly  ,
    assembler_iters : u64 ,
    mutator   : Mutation  ,
    searcher : LocalSearch, 
}

impl MemeticBagTsp {
    fn run(mut self) {
        let mut rng = self.rng;
        let p = &self.problem;
        let tmax = p.tmax;
        let costs = &p.costs;
        let problem_len = self.problem.len;
        let elite_size  = self.elite_size;
        let pop_size    = self.pop_size;

        let aiters = self.assembler_iters;

        let mut pop: Box<_> = (self.genesis) (
            problem_len,
            pop_size
        ).into_iter()
         .map(|tour| {
             let fit = (self.fitness) (p, tour.as_slice());
             (tour, fit)
         }).collect();

        let mut used = BitSet::with_capacity(problem_len);

        // by max fit
        loop {
            pop.sort_by(|(_, l), (_, r)| r.total_cmp(l));
            for (t, _) in &pop {
                used.union_with(t.visited());
            }

            // avaiable.toggle_range(..);
            let (elite, others) = pop[..pop_size]
                .split_at_mut(elite_size);

            for (t, _) in others.iter_mut() {
                let l = rng.random_range(0..elite_size);
                let r = rng.random_range(0..elite_size);

                let l = elite[l].0.visited_block();
                let r = elite[r].0.visited_block();
                let child = t.visited_block_mut();

                (self.crosser)(l, r, child, &mut rng);
                t.fix();
            }

            for (t, f) in others.iter_mut() {
                (self.assembler)(
                    t, &mut rng,
                    costs, aiters
                );

                *f = (self.fitness)(p, t.as_slice());
            }

            for (t, f) in pop.iter_mut() {
                (self.mutator) (t, &mut rng, &used, costs, tmax);
                *f = (self.searcher) ();
            }

            used.clear();
        }
    }
}

fn pop_genesis(
    problem_len: usize,
    pop_size   : usize
) -> Box<[Tour]> {
    assert!(pop_size % 2 == 0);

    let pairs = pop_size / 2;
    let mut pop = Vec::with_capacity(pairs);
    for _ in 0..pairs {
        let (fst, snd) = birth_two(problem_len);
        pop.push(fst);
        pop.push(snd);
    }
    
    pop.into_boxed_slice()
}

fn birth_two(
    problem_len: usize
) -> (Tour, Tour) {
    todo!()
}

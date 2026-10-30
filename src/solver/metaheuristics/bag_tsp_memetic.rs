
mod cross;

use std::mem::MaybeUninit;
use fixedbitset::FixedBitSet as BitSet;
use grid::Grid;
use rand::{
    *,
    rngs::SmallRng,
};

use crate::{
    Problem, Tour, problem::{
        Point, TourStatus, 
    },
};

type Genesis  = fn(problem_size: usize, pop_size: usize) -> Box<[Tour]>;
type Crossing = fn(
    l    : &[usize], // the bytes 
    r    : &[usize], // the byts
    child: &mut [usize],
    rng  : &mut SmallRng, 
);

type Renovation = fn(&Box<Tour>, SmallRng);
type Fitness    = fn(TourStatus) -> f64;
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
type LocalSearch = fn(
    tour   : &mut Tour,
    used   : &mut u64,
    budget : u64,
    problem: &Problem,
) -> TourStatus; // (fit, cost)

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
    fn run (self) -> Tour {
        let mut used = 0;
        let budget = self.budget;
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
            // by max fit
            pop.sort_by(|(_, l, _), (_, r, _)| r.total_cmp(l));
            if let Some((tour, _, ts)) = extracts_best_feasible(
                &pop, best_score, tmax,
            ) && ts.score > best_score {
                best_tour = tour.clone();
                best_score = ts.score;
            }

            for (t, _, _) in &pop {
                occupied.union_with(t.visited());
            }

            // avaiable.toggle_range(..);
            let (elite, others) = pop[..pop_size]
                .split_at_mut(elite_size);

            for (t, f, ts) in others.iter_mut() {
                let l = rng.random_range(0..elite_size);
                let r = rng.random_range(0..elite_size);

                let l = elite[l].0.visited_block();
                let r = elite[r].0.visited_block();
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

// assumes its sorted
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

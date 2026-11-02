
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

pub type Fitness =
    Box<dyn Fn(TourStatus) -> f64>;

type Assembly   = fn(
    &mut [Point], &mut SmallRng, 
    costs: &Grid<f64>,
    max_iters: u64
) -> f64; // cost


pub struct MemeticBagTsp {
    // problem         : Problem   ,
    // budget          : u64       ,
    genesis         : Genesis   ,
    pop_size        : usize     ,
    elite_size      : usize     ,
    round_len       : usize     , 
    // fitness         : Fitness   ,
    // rng             : SmallRng  ,
    crosser         : Crossing  , 
    assembler       : Assembly  ,
    tsp_searcher    : TspSearch, 
    tsp_iters       : u64 ,
    mutator         : Mutation  ,
    searcher        : LocalSearch, 
}

impl MemeticBagTsp {
    fn run (
        &mut self       ,
        p     : &Problem,
        mut rng   : SmallRng,
        budget: u64     ,
    ) -> (Tour, u64) {
        let mut used    = 0               ;
        let tmax        = p.tmax          ;
        let costs       = &p.costs        ;
        let scores      = &p.scores       ;
        let problem_len = p.len;
        let elite_size  = self.elite_size ;
        let round_len   = self.round_len  ;
        let pop_size    = self.pop_size   ;

        let tsp_iters = self.tsp_iters;
        let mut chairs = vec![
            Tour::new(problem_len);
            elite_size
        ].into_boxed_slice();

        let fitness: Fitness = Box::new(
            move |status: TourStatus| {
                let score = status.score as f64;
                
                if status.cost <= tmax {
                    score
                } else {
                    score * (tmax / status.cost).powi(4)
                }
            }
        );

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

             let fit = fitness (ts);
             (tour, fit, ts)
         }).collect();

        let mut occupied   = BitSet::with_capacity(problem_len);
        let mut best_tour  = Tour::new(problem_len);
        let mut best_score = 0;

        let mut iters = 0u64;
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

                ts.cost = (self.assembler)(
                    t.as_mut_slice(), &mut rng,
                    costs, tsp_iters
                );
                ts.score = t.score(scores);

                *ts = (self.mutator) (
                    t,
                    *ts, 
                    &mut rng,
                    &occupied,
                    &mut used,
                    budget, 
                    costs,
                    scores, 
                    // tmax,
                );

                *f = fitness(*ts);
                (*ts, *f) = (self.searcher) (
                    t, *ts, *f, 
                    &mut rng, &mut used, budget,
                    p, &fitness, self.tsp_searcher,tsp_iters,
                );

                if ts.cost <= tmax && ts.score > best_score {
                    best_score = ts.score;
                    best_tour = t.clone();
                }

                if used >= budget { return (best_tour, iters) }
            }

            occupied.clear();
            iters += 1;
        }

        (best_tour, iters)
    }
}
use std::time::Instant;
use itertools::iproduct;
use crate::{Method, experiment::Result as EResult};

use super::tsp::ils;
impl Method for MemeticBagTsp {
    fn shot(&mut self, p: &Problem, rng: SmallRng, budget: u64) -> EResult {
        let start = Instant::now();
        let (tour, iters) = MemeticBagTsp::run(self, p, rng, budget);
        let time = start.elapsed();

        EResult {
            tour, time, iters,
        }
    }

    fn all_ms() -> Vec<(Self, String)> {
        let mut v = Vec::new();
        for (pop, elite, round, tspiters) in iproduct!(
            POP_SIZES ,
            ELITE_SIZES,
            ROUND_SIZES,
            TSPITERS  ,
        ) {
            let m = MemeticBagTsp {
                genesis          : pop_genesis   ,
                pop_size         : pop           ,
                elite_size       : elite         ,
                round_len        : round         ,
                // fitness          : todo!()       ,
                // rng              : rng           ,
                crosser          : bag_cross     ,
                assembler        : ils::assembly ,
                tsp_searcher     : ils::ils      ,
                tsp_iters        : tspiters      ,
                mutator          : inserts_unused,
                searcher         : canonical     ,
            };

            let m_name = format!("memetic_bag_tsp_popsize:{pop}_elite_size:{elite}_round_len:{round}_tspiters:{tspiters}");

            v.push((m, m_name));
        }

        v
    }
}

const POP_SIZES  : [usize;3] = [ 20, 30, 50 ];
const ELITE_SIZES: [usize;3] = [ 5 , 7 , 10 ];
const ROUND_SIZES: [usize;3] = [ 3 , 5 , 8 ];
const TSPITERS: [u64;3] = [ 1_000 , 10_000 , 100_000 ];

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
    for chair in chairs {
        let winner = round(rng, round_len, pop);
        chair.extend_from_slice(winner);
    }
}


fn round<'a> (
    rng: &mut SmallRng,
    round_len: usize,
    pop: &'a[(Tour, f64, TourStatus)],
) -> &'a [Point]{
    let pop_len = pop.len();

    // dont need to check round_len > 1 omg!
    let mut i = rng.random_range(0..pop_len);
    let (mut best_idx, mut best_fit) = (i, pop[i].1);
    for _ in 1..round_len {
        i = rng.random_range(0..pop_len);
        if pop[i].1 > best_fit {
            best_idx = i;
            best_fit = pop[i].1;
        }
    }

    &pop[best_idx].0.as_slice()
}


pub fn fitness(
    status: TourStatus,
    tmax: f64,
    // beta: i32,
) -> f64 {
    let score = status.score as f64;

    if status.cost <= tmax {
        score
    } else {
        score * (tmax / status.cost).powi(4) // lets just use 4
    }
}

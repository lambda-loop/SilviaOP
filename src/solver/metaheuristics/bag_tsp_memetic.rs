
mod cross;

use std::mem::MaybeUninit;
use fixedbitset::FixedBitSet as BitSet;
use rand::{
    *,
    rngs::SmallRng,
};

use crate::{
    Problem, Tour,
};

type Genesis  = fn(problem_size: usize, pop_size: usize) -> Box<[Tour]>;
type Crossing = fn(
    l    : &[usize], // the bytes 
    r    : &[usize], // the byts
    child: &mut [usize],
    rng  : &mut SmallRng, 
);

type Renovation = fn(&Box<Tour>, SmallRng);
type Fitness    = fn(&Problem, Tour) -> (Tour, f64); 

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
}

impl MemeticBagTsp {
    fn run(mut self) {
        let mut rng = self.rng;
        let p = &self.problem;
        let problem_len = self.problem.len;
        let elite_size  = self.elite_size;
        let pop_size    = self.pop_size;

        let mut pop: Box<_> = (self.genesis) (
            problem_len,
            pop_size
        ).into_iter()
         .map(|tour| (self.fitness) (p, tour))
         .collect();

        let mut avaiable = BitSet::with_capacity(problem_len);
        // by max fit
        loop {
            pop.sort_by(|(_, l), (_, r)| r.total_cmp(l));
            for (t, _) in &pop {
                avaiable.union_with(t.visited());
            }

            avaiable.toggle_range(..);
            let (elite, others) = pop[..pop_size]
                .split_at_mut(elite_size);

            for (t, _) in others {
                let l = rng.random_range(0..elite_size);
                let r = rng.random_range(0..elite_size);

                let l = elite[l].0.visited_block();
                let r = elite[r].0.visited_block();
                let child = t.visited_block_mut();

                (self.crosser)(l, r, child, &mut rng);
            }

            avaiable.clear();
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


use rand::{
    *,
    rngs::SmallRng,
};

use crate::problem::Tour;

pub type Genesis  = fn(
    problem_size: usize   ,
    rng         : &mut SmallRng,
    pop_size    : usize   ,
) -> Box<[Tour]>;

pub fn pop_genesis(
    problem_len: usize,
    rng        : &mut SmallRng,
    pop_size   : usize
) -> Box<[Tour]> {
    debug_assert!(pop_size % 2 == 0);
    debug_assert!(pop_size > 0);

    let pairs = pop_size / 2;
    let mut pop = Vec::with_capacity(pop_size);
    for _ in 0..pairs {
        let (fst, snd) = birth_two(problem_len, rng);
        pop.push(fst);
        pop.push(snd);
    }
    
    pop.into_boxed_slice()
}

fn birth_two(
    problem_len: usize,
    rng: &mut SmallRng,
) -> (Tour, Tour) {
    let mut l = Tour::new(problem_len);
    let mut r = Tour::new(problem_len);

    let l_blocks = l.visited_block_mut();
    let r_blocks = r.visited_block_mut();
    for (lb, rb) in l_blocks.iter_mut()
        .zip(r_blocks.iter_mut()) {
        *lb = rng.next_u64() as usize;
        *rb = !*lb;
    }
    l.fix();
    r.fix();
    (l, r)
}

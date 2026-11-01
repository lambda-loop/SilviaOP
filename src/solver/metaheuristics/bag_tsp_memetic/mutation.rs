
use rand::{
    *,
    seq::IteratorRandom,
    rngs::SmallRng,
};
use fixedbitset::FixedBitSet as BitSet;
use grid::Grid;
use crate::{
    problem::{
        tour::*,
    },
};

pub type Mutation   = fn(
    tour   : &mut Tour    ,
    status : TourStatus   ,
    rng    : &mut SmallRng,
    visited: &BitSet      ,
    used   : &mut u64,
    budget : u64,
    costs  : &Grid<f64>   ,
    scores : &[u16]       ,
) -> TourStatus; 

pub fn inserts_unused(
    tour   : &mut Tour    ,
    mut status : TourStatus   ,
    rng    : &mut SmallRng,
    visited: &BitSet      ,
    used   : &mut u64,
    budget : u64,
    costs  : &Grid<f64>   ,
    scores : &[u16]       ,
) -> TourStatus {
    if *used >= budget { return status }
    if let Some(p) = visited.zeroes().choose(rng) {
        tour.push(p as Point);
        status.score += scores[p as usize] as u32;
        let tour_len = tour.len();
        let last = if tour_len == 1 { 0 } else {
            tour[tour_len as usize - 2] as usize
        };

        status.cost += 
            costs[(last, p as usize)] +
            costs[(p as usize, 1)]    -
            costs[(last, 1)];
        *used += 1;
    }

    status
}

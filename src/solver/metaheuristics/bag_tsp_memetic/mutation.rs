
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
    tour : &mut Tour,
    rng  : &mut SmallRng,
    used : &BitSet,
    costs: &Grid<f64>,
    tmax : f64,
); 

pub fn inserts_unused(
    tour : &mut Tour,
    rng  : &mut SmallRng,
    used : &BitSet,
    _costs: &Grid<f64>,
    _tmax : f64,
) {
    if let Some(p) = used.zeroes().choose(rng) {
        tour.push(p as Point);
        
    }
}

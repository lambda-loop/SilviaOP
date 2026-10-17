

use rand::{
    *,
    rngs::ThreadRng,
};
use fixedbitset::FixedBitSet as BitSet;

use crate::problem::{
    Problem,
    TourStatus,
};

pub type Mutator = Box<
    dyn Fn(
        &Problem,
        &[u8],
        TourStatus,
        &mut BitSet,
        &mut ThreadRng,
    ) -> Vec<u8>,
>;

pub fn adaptative(
    p: &Problem,
    tour: &[u8],
    status: TourStatus,
    visited: &mut BitSet,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    if status.cost > p.tmax {
        random_remotion(tour, visited, rng)
    } else {
        random_insertion(tour, visited, rng)
    }
}

fn random_insertion(
    tour: &[u8],
    visited: &mut BitSet,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    let tour_len = tour.len();
    let zeroes = visited.count_zeroes(2..);
    if zeroes == 0  { return tour.to_vec() }

    let idx = rng.random_range(0..=tour_len);
    let mut tour_ = Vec::with_capacity(tour_len + 1);

    tour_.extend_from_slice(&tour[..idx]);
    let index = rng.random_range(0..zeroes);
    let vertice = visited
        .zeroes()
        .nth(index)
        .unwrap();

    tour_.push(vertice as u8);
    tour_.extend_from_slice(&tour[idx..]);

    visited.insert(vertice);
    tour_
}

fn random_remotion (
    tour: &[u8],
    visited: &mut BitSet,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    let tour_len = tour.len();
    if tour_len == 0 { return Vec::new() }

    let idx = rng.random_range(0..tour_len);
    let mut tour_ = Vec::with_capacity(tour_len - 1);

    tour_.extend_from_slice(&tour[..idx]);
    tour_.extend_from_slice(&tour[idx+1..]);

    visited.remove(tour[idx] as usize);
    tour_
}







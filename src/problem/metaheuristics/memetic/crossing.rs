
use crate::problem::route::TourStatus;
use rand::{
    *, rngs::ThreadRng,
};

use fixedbitset::FixedBitSet as BitSet;

pub type MatchMaker = Box<
    dyn for<'pop> Fn(
        &'pop [(Vec<u8>, f32, TourStatus)],
        &mut ThreadRng,
    ) -> (&'pop [u8], &'pop [u8])
        >;
pub type Mate = Box<dyn Fn(&[u8], &[u8], usize, &mut ThreadRng) -> Vec<u8>>;

use std::cmp::{min, max};
pub fn rejoin(
    l: &[u8],
    r: &[u8],
    p_len: usize,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    if l.is_empty() { return r.to_vec(); }
    if r.is_empty() { return l.to_vec(); }

    let mut visited = BitSet::with_capacity(p_len);
    visited.insert_range(0..2);

    let (l_len, r_len) = (l.len(), r.len());
    let begin = rng.random_range(0..l_len);
    let end = rng.random_range(begin + 1..=l_len);

    let l_part = &l[begin..end];
    for &v in l_part { visited.insert(v as usize) }

    let mut child = Vec::with_capacity( max(l_len, r_len) );
    for i in 0..min(begin, r_len) {
        let v = r[i];
        if !visited.contains(v as usize) {
            child.push(v);
        }
    }

    child.extend_from_slice(l_part);
    for i in end..r_len {
        let v = r[i];
        if !visited.contains(v as usize) {
            child.push(v);
        }
        
    }

    child
}

pub fn random_match<'pop> (
    pop: &'pop [(Vec<u8>, f32, TourStatus)],
    rng: &mut ThreadRng,
) -> (&'pop [u8], &'pop [u8]) {
    let pop_len = pop.len();
    let i = rng.random_range(0..pop_len);
    let j = rng.random_range(0..pop_len);
    (&pop[i].0, &pop[j].0)
}

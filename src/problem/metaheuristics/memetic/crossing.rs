
use crate::problem::route::TourStatus;
use rand::{
    *, rngs::ThreadRng,
};

pub type MatchMaker = Box<
    dyn for<'pop> Fn(
        &'pop [(Vec<u8>, f32, TourStatus)],
        &mut ThreadRng,
    ) -> (&'pop [u8], &'pop [u8])
        >;
pub type Mate = Box<dyn Fn(&[u8], &[u8], &mut ThreadRng) -> Vec<u8>>;

pub fn rejoin(
    l: &[u8], // thats a parent
    r: &[u8], // thats a parent
    rng: &mut ThreadRng
) -> Vec<u8> {
    if l.is_empty() { return r.to_vec() }
    if r.is_empty() { return l.to_vec() }
    let l_len = l.len();
    let r_len = r.len();

    let l_idx = rng.random_range(0..=l_len/2);
    let r_idx = rng.random_range(r_len/2..r_len);

    let mut child = Vec::with_capacity(l_len + r_len - r_idx);

    child.extend_from_slice(&l[..l_idx]);
    child.extend_from_slice(&r[r_idx..]);

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

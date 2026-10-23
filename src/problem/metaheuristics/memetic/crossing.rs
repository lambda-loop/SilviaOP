
use crate::problem::{
    Tour, Point,
};

use crate::problem::route::TourStatus;
use rand::{
    *, rngs::ThreadRng,
};

use fixedbitset::FixedBitSet as BitSet;

pub type MatchMaker = Box<
    dyn for<'pop> Fn(
        &'pop [(Tour, f32, TourStatus)],
        &mut ThreadRng,
    ) -> (&'pop [Point], &'pop [Point])
        >;

pub type Mate = Box<dyn Fn(
    &[Point], // l 
    &[Point], // r
    &mut Tour, // r#in
    usize    , // p_len
    &mut ThreadRng
)>;

use std::cmp::{min, max};
pub fn rejoin(
    l   : &[Point],
    r   : &[Point],
    r#in: &mut Tour,
    p_len: usize,
    rng: &mut ThreadRng,
) {
    if l.is_empty() { r#in.under(r) }
    else if r.is_empty() { r#in.under(l) }
    else {

        let mut visited = BitSet::with_capacity(p_len);
        visited.insert_range(0..2);

        let (l_len, r_len) = (l.len(), r.len());
        let begin = rng.random_range(0..l_len);
        let end   = rng.random_range(begin + 1..=l_len);

        let l_part = &l[begin..end];
        for &v in l_part { visited.insert(v as usize) }

        // let mut child = Tour::new(p_len);
        r#in.reset();
        for i in 0..min(begin, r_len) {
            let v = r[i];
            if !visited.contains(v as usize) {
                r#in.push(v);
            }
        }

        // TODO: extend from slice
        // child.extend_from_slice(l_part);
        for &i in l_part {
            r#in.push(i)
        }

        for i in end..r_len {
            let v = r[i];
            r#in.push(v);
        }
    }
}

pub fn random_match<'pop> (
    pop: &'pop [(Tour, f32, TourStatus)],
    rng: &mut ThreadRng,
) -> (&'pop [Point], &'pop [Point]) {
    let pop_len = pop.len();
    let i = rng.random_range(0..pop_len);
    let j = rng.random_range(0..pop_len);
    (&pop[i].0, &pop[j].0)
}

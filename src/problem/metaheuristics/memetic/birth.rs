
use std::thread::Thread;
use crate::problem::{
    Tour, Point,
};

use rand::{
    *, rngs::ThreadRng, // seq::SliceRandomg,
    seq::SliceRandom,
};

pub type Birth = Box<dyn Fn(usize, usize, &mut ThreadRng) -> Vec<Vec<u8>>>;

use fixedbitset::FixedBitSet as BitSet;

// pub fn simple(
//     p_len: usize,
//     size: usize,
//     rng: &mut ThreadRng,
// ) -> Vec<Tour> {
//     assert_eq!(size % 2, 0);

//     let mut r#gen = Vec::with_capacity(size);
//     for _ in 0..size/2 {
//         let (a, b) = birth(p_len, rng);
//         r#gen.push(a);
//         r#gen.push(b);
        
//     }

//     r#gen
// }

fn birth (
    p_len: usize,
    rng: &mut ThreadRng,
) -> (Tour, Tour) {
    let mut vs = BitSet::with_capacity(p_len);

    for _ in 0..p_len/2 {
        let v = rng.random_range(2..p_len);
        vs.insert(v)
    }

    let mut l = Tour::new(p_len);
    let mut r = Tour::new(p_len);

    for i in 2..p_len {
        if vs[i] {
            l.push(i as Point);
        } else {
            r.push(i as Point);
        }
    }

    (l, r)
}


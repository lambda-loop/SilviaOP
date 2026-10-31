
use rand::{
    *, rngs::SmallRng,
};

use fixedbitset::{
    FixedBitSet as BitSet,
    Block,
};


pub type Crossing = fn(
    l    : &[usize], // the bytes 
    r    : &[usize], // the byts
    child: &mut [usize],
    rng  : &mut SmallRng, 
);

pub fn bag_cross(
    l    : &[usize], // the bytes 
    r    : &[usize], // the byts
    child: &mut [usize],
    rng  : &mut SmallRng, 
) {
    debug_assert_eq!(l.len(), r.len());
    debug_assert_eq!(l.len(), child.len());

    for ((out, &left), &right) in
        child.iter_mut().zip(l).zip(r) {
        let mask = rng.next_u64() as usize;
        *out = (left & mask) | (right & !mask);
    }
}
    

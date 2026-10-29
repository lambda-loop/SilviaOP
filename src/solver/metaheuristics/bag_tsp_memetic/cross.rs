
use rand::{
    *, rngs::SmallRng,
};

use fixedbitset::{
    FixedBitSet as BitSet,
    Block,
};

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
    
// fn gen_mask (
//     mask: &mut [usize],
//     rng : &mut SmallRng,
// ) {
//     let rem = mask.len() % Block::BITS as usize;
//     let restantes = mask.len() % Block::BITS as usize;

//     for block in mask.iter_mut() {
//         *block = rng.next_u64() as Block;
//     }

//     // Puts zero in the unbounded bits
//     if rem != 0 {
//         *mask.last_mut().unwrap() &= ((1 as Block) << restantes) - 1;
//     }
// }

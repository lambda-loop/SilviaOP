
pub mod two_opt;

use crate::Tour;

// A simple and incremental implementation
// 1 -> ILS + 2-opt	
// 2 -> ILS + 2-opt + relocate	
// 3 -> ILS + 2-opt + Or-opt	

pub fn ils(tour: &mut Tour, iters: u64) {
    for _ in 0..iters {
        
    }
}

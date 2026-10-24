//

pub mod two_opt;
// pub mod insert;

use rand::{
    rngs::ThreadRng,
};

use crate::{
    OutOfCredits, problem::{
        Problem,
        route::*,
    }
};


pub type LocalSearcher =
    Box<dyn Fn(&mut LocalSearchCtx)
               -> Result<(), OutOfCredits>>;

pub struct LocalSearchCtx<'a> {
    pub p            : &'a Problem          ,
    pub tour         : &'a mut Tour         ,
    pub score        : u64                  ,
    pub cost         : f64                  ,
    pub fit          : &'a f32              ,
    pub used         : &'a mut u64          ,
    pub budget       : u64                  ,
    pub iters: usize, // amount of tours to generate
    pub rng          : &'a mut ThreadRng    ,
    pub fitness      : Box<dyn Fn(TourStatus) -> f32 + 'a>,
    // pub visited      : BitSet
}



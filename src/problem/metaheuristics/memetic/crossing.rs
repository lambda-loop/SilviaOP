

use crate::problem::route::TourStatus;

pub type MatchMaker = Box<dyn Fn(&[(Vec<u8>, f32, TourStatus)]) -> (&[u8], &[u8])>;
pub type Mate = Box<dyn Fn((&[u8], &[u8])) -> Vec<u8>>;



use rand::rngs::ThreadRng;
pub type Mutator = Box<dyn Fn(&mut ThreadRng, Vec<u8>) -> Vec<u8>>;


use grid::Grid;
use fixedbitset::FixedBitSet as BitSet;

/* so "

let mut tour = Tour::new(100);
tour.push(5);
tour.push(9);

evaluate(&tour); // prints [5, 9]

" works */

// general usage
pub type Point = u16;

// TODO: use maybeUnit so Tour<T> would be better.
#[derive(Debug, Clone)]
pub struct Tour {
    buffer: Box<[Point]>,
    // TODO: check if its on heap or stack
    visited: BitSet, 
    // the max of existing points
    // is also the same as the max buffer len
    // (of course)
    len: Point, 
}

impl Tour {
    // excluding 0 and 1
    pub fn new(n_points: usize) -> Self {
        assert!((2..=usize::from(Point::MAX) + 1).contains(&n_points));
        // if n_points >= Point::MAX as usize {
        //     panic!("handling more points then possible currently!")
        // }
        let buffer = unsafe {
            Box::<[Point]>::new_zeroed_slice(n_points - 2).assume_init()
        };

        let mut visited = BitSet::with_capacity(n_points);
        visited.insert_range(0..2);

        Tour {
            buffer, visited, 
            len: 0,
        }
    }

    #[inline(always)]
    pub fn len(&self) -> u16 {
        self.len
    }

    pub fn push(&mut self, point: Point) {
        // its a new point
        if !self.visited.put(point as usize) {
            unsafe {
                *self.buffer.get_unchecked_mut(
                    self.len as usize
                ) = point;
            }
            self.len += 1;
        }
    }

    pub fn as_slice(&self) -> &[Point] {
        &self.buffer[0..self.len as usize]
    }

    pub fn as_mut_slice(&mut self) -> &mut [Point] {
        &mut self.buffer[0..self.len as usize]
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // u kinda "clone" something but OVER another instance
    pub fn over(&self, under: &mut Self) {}

    // changes based in the "over" slice
    pub fn under(&mut self, over: &[Point]) {}

    // it will be workst than a personalized version
    // but will work for save allocations
    pub fn reset(&mut self) {
        self.len = 0;
        self.visited.clear();
        self.visited.insert_range(0..2);
    }

    pub fn reverse_in(
        &mut self   ,
        begin: usize,
        end  : usize, 
    ) {
        assert!(begin <= end && end < usize::from(self.len));
        self.buffer[begin..=end].reverse();
    }

    #[inline(always)]
    pub fn visited_block(&self) -> &[usize] {
        self.visited.as_slice()
    }

    #[inline(always)]
    pub fn visited_block_mut (&mut self) -> &mut [usize] {
        self.visited.as_mut_slice()
    }

    #[inline(always)]
    pub fn visited (&self) -> &BitSet {
        &self.visited
    }

    pub fn avaiable<'a> (&'a self) -> fixedbitset::Zeroes<'a> {
        self.visited.zeroes()
    }

    // if only the visited matters and the visited was altered in a nom proper way.. Then we need to
    // create some tour (probably bad) that matches the visited list
    pub fn fix(&mut self) {}
    pub fn cost(&self, costs: &Grid<f64>) -> f64 {
        todo!()
    }

    pub fn score(&self, scores: &[u16]) -> u32 {
        todo!()
    }

}

impl std::ops::Deref for Tour {
    type Target = [Point];

    fn deref(&self) -> &[Point] {
        &self.buffer[..self.len as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TourStatus {
    pub cost: f64,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct RouteStatus {
    pub total_score: u32,
    pub total_consume: f64,
}

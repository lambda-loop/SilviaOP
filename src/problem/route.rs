//

use fixedbitset::FixedBitSet as BitSet;

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
        if n_points >= Point::MAX as usize {
            panic!("handling more points then possible currently!")
        }
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

}

/* so "

let mut tour = Tour::new(100);
tour.push(5);
tour.push(9);

evaluate(&tour); // imprime [5, 9]

" works */

impl std::ops::Deref for Tour {
    type Target = [Point];

    fn deref(&self) -> &[Point] {
        &self.buffer[..self.len as usize]
    }
}


#[derive(Debug, Clone, Copy)]
pub struct TourStatus {
    pub cost: f32,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct RouteStatus {
    pub total_score: u32,
    pub total_consume: f32,
}

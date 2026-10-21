
use grid::Grid;
use fixedbitset::FixedBitSet as BitSet;
use macroquad::input::KeyCode::O;

use crate::problem::{
    Problem,
    TourStatus,
};

pub struct ACO {
    // params
    alpha: f32,
    beta : f32,
    // taff : Grid<f32>,

    local_search: Box<dyn Fn(Vec<u8>, &mut u64, u64) -> (Vec<u8>, TourStatus)>,
    rho  : f32,
    Q    : f32,
    t0   : f32,
    num_ants: u32,
}

impl ACO {
    fn run(&self, p: &Problem, budget: u64) -> Vec<u8> {
        let mut used = 0u64;
        let mut taff = Grid::init(p.len, p.len, self.t0);

        let mut best_tour  = Vec::new();
        let mut best_score = 0u32;

        while used < budget {
            for _ in 0..self.num_ants {
                let mut tour = Vec::new();
                let mut visited  = BitSet::with_capacity(p.len);
                visited.insert_range(0..2);
                
                let mut current = 0usize;
                let mut cost    = p.costs[(0, 1)];

                loop {
                    let candidates = visited
                        .zeroes()
                        .map(|j| {
                            used += 1;
                            let next_cost = cost
                                - p.costs[(current, 1)]
                                + p.costs[(current, j)]
                                + p.costs[(j, 1)];

                            (j, next_cost)
                        })
                        .filter(|&(_, cost)| cost <= p.tmax);

                    // if its empty..
                    let mut sum = 0.;
                    let candidates: Vec<_> = candidates
                        .map(|(j, cost)| {
                            let eta_j = p.scores[j] as f32
                                / p.costs[(current, j)].max(f32::EPSILON);
                            let weight = taff[(current, j)].powf(self.alpha)
                                    * eta_j.powf(self.beta);
                            sum += weight;
                            (j, cost, weight)
                        }).collect();
                    if used > budget { return best_tour }
                    if candidates.is_empty() { break }


                    let (chosen_j, chosen_cost) = if sum > 0. {
                        raffle_roulette(candidates)
                    } else {
                        raffle_uniform(candidates)
                    };

                    tour.push(chosen_j);
                    visited.insert(chosen_j as usize);
                    cost = chosen_cost;
                    current = chosen_j as usize;
                }

                    let (tour, status) = (self.local_search)(tour, &mut used, budget);
                    if status.score > best_score {
                        best_score = status.score;
                        best_tour = tour;
                    }
            }

            evaporation(&mut taff, self.rho);
            reinforcement(&mut taff, &best_tour, best_score, self.Q);
        }

        best_tour
    }
}

fn raffle_uniform (candidates: Vec<(usize, f32, f32)>) -> (u8, f32) {
    todo!()
}

fn raffle_roulette (candidates: Vec<(usize, f32, f32)>) -> (u8, f32) {
    todo!()
}

fn evaporation(taff: &mut Grid<f32>, rho: f32) {}
fn reinforcement (taff: &mut Grid<f32>, tour: &[u8], score: u32, Q: f32) {}

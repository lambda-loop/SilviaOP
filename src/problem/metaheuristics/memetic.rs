

use rand::{
    *, rngs::ThreadRng,
};

mod local_search;
mod mutation;
mod crossing;
// use std::collections::HashSet;

use mutation::*;
use crossing::*;
use local_search::*;
use crossing::{
    MatchMaker, Mate,
};

use fixedbitset::FixedBitSet as BitSet;

use crate::{
    OutOfCredits, problem::{
        Problem, route::TourStatus,
    },
};

pub struct Memetic {
    pub p       : Problem  ,
    pub budget  : u64,

    // builder alpha for the grasp
    pub alpha   : f32      , 
    // fitness paramter from the paper
    pub beta    : i32      ,
    pub rng     : ThreadRng,

    pub pop_size: usize    ,
    pub cut_size: usize    ,
    pub neighborhood_size: usize    ,

    pub grasp: Grasp,
    pub match_maker : MatchMaker,
    pub mate        : Mate,
    pub mutator : Mutator  ,
    pub searcher: LocalSearcher,

    
}

// TODO: move it and adjust it
pub type Grasp = fn(
    &Problem,           // p
    &mut u64,           // used
    u64,                // budget
    f32,                // alpha 
    &mut ThreadRng      // rng
) -> Result<Vec<u8>, OutOfCredits>;

fn simple (m: &mut Memetic) -> Vec<u8> {
    assert!(0 < m.cut_size);
    assert!(m.cut_size < m.pop_size);

    let mut rng = rand::rng();

    let mut used = 0u64;
    let p = &m.p;
    let beta = m.beta;
    
    let fit = move |status: TourStatus| -> f32 {
        fitness(p, status, beta)
    };

    let pop_size = m.pop_size;
    let mut pop = Vec::with_capacity(m.pop_size);
    for _ in 0..pop_size {
        let tour = (m.grasp)(
            &m.p,
            &mut used,
            m.budget,
            m.alpha,
            &mut m.rng,
        ).expect(
            "GRASP has failed during its build"
        );
        pop.push(tour);
    }

    let mut best_tour = pop[0].clone();
    let mut best_score = m.p.eval_tour(
        &mut used,
        m.budget,
        &best_tour
    ).unwrap().score;
    // theres no way all the budget will be used
   // in THE FIRST STEP STUPID AIs!

    let mut pop_ = Vec::with_capacity(m.pop_size);
    for i in 0..pop_size {
        let tour = pop[i].clone(); // TODO: fix, its slow (not for ai)
        let Some(status) = m.p.eval_tour(
            &mut used,
            m.budget,
            &tour) else { break };

        let fit = fit(status);
        pop_.push((tour, fit, status));
    }

    loop {
        pop_.sort_by(|(_, l, _), (_, r, _)| {
            r.total_cmp(&l) // reverted
        });
        
       updates_best(
            &m.p,
            &mut best_tour,
            &mut best_score,
            &pop_[..],
        );
        if pop_.len() <= m.cut_size { return best_tour; }

        let (parents, children) = pop_.split_at_mut(m.cut_size);
        for child in children.iter_mut() {
            // SELECTIOn
            let (pl, pr) = (m.match_maker)(parents, &mut rng);

            // MATE
            let new_child = (m.mate) (pl, pr, &mut rng);
            let Some(status) = m.p.eval_tour(&mut used, m.budget, &new_child)
            else { return best_tour };

            if status.cost <= m.p.tmax && status.score > best_score {
                best_score = status.score;
                best_tour = new_child.clone();
            }

            let mut visited = new_visited(&m.p, &new_child);

            // MUTATION
            let new_child = (m.mutator)(
                &m.p,
                &new_child,
                status,
                &mut visited,
                &mut m.rng, 
            );
            
            let Some(status) = m.p.eval_tour(
                &mut used,
                m.budget,
                &new_child,
            ) else { return best_tour; };

            if status.cost <= m.p.tmax && status.score > best_score {
                best_score = status.score;
                best_tour = new_child.clone();
            } let fitness = fit(status);

            let mut ctx = LocalSearchCtx {
                p: &m.p,
                tour: new_child,
                status, fit: fitness,
                budget: m.budget,
                used: &mut used,
                neighborhood: m.neighborhood_size,
                rng: &mut m.rng,
                fitness: Box::new(fit), 
                visited,
            };

            // LOCAL SEARCH
            match (m.searcher)(&mut ctx) {
                Err(_) => return best_tour,
                Ok((well_educated_child, status, fit)) => {
                    if status.cost <= m.p.tmax && status.score > best_score {
                        best_score = status.score;
                        best_tour = well_educated_child.clone();
                    }

                    *child = (well_educated_child, fit, status);
                },
            }
        }
    }
}

// performs both of the things: cuts and reproduce
// fn select(mut tours: Vec<Vec<u8>>) -> Vec<Vec<u8>> {

    
    
// }

// static fitness
fn fitness(p: &Problem, status: TourStatus, beta: i32) -> f32 {

    let score = status.score as f32;
    let cost = status.cost;
    if cost <= p.tmax { score }
    else {
        score * (p.tmax / cost).powi(beta as i32)
    }
}
 // h  ← 100                  // tamanho da população
 //    pc ← 0,70                 // probabilidade de cruzamento
 //    pm ← 0,40                 // probabilidade de mutação
 //    α  ← 4                    // intensidade da penalização

// assumes that its ordered
// trys to change a current best to a new FEASIBLE one

fn updates_best(
    p: &Problem,
    // used: &mut usize,
    // budget: usize,
    best_tour: &mut Vec<u8>,
    best_score: &mut u32,
    pop: &[(Vec<u8>, f32, TourStatus)] // assumed to be sorted
) {
    for (tour, _, status) in pop {
        if status.score <= *best_score { return () }
        if status.cost > p.tmax { continue }

        *best_score = status.score;
        *best_tour = tour.clone();
        return ()
        // return Ok(());
    } 

    // Ok(())
}

pub fn update_visited(
    tour: &[u8],
    visited: &mut BitSet
) {
    visited.set_range(2.., false);
    for &v in tour {
        visited.insert(v as usize);
    }
}

fn new_visited(p: &Problem, tour: &[u8]) -> BitSet {
    let mut visited = BitSet::with_capacity(p.len);
    visited.insert_range(0..2);
    for &v in tour {
        visited.insert(v as usize);
    }
    visited
}




mod local_search;
mod mutation;
mod crossing;
// use std::collections::HashSet;

use mutation::*;
use crossing::*;
use local_search::*;
use rand::{*, rngs::ThreadRng};

use fixedbitset::FixedBitSet as BitSet;

use crate::{
    OutOfCredits, problem::{
        Problem, route::TourStatus,
    },
};

use crate::metaheuristics::grasp::build::Builder;
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

use std::{rc::Rc, cell::Cell};
fn simple (m: &mut Memetic) -> Result<Vec<u8>, OutOfCredits>{
    assert!(0 < m.cut_size);
    assert!(m.cut_size < m.pop_size);
    // let used = Rc::new( 
    //     Cell::new(0u64)
    // );
    let mut used = 0u64;
    let p = &m.p;
    let beta = m.beta;
    
    let used_clone = used.clone();
    let fit = move |status: TourStatus| -> f32 {
        // let status = p.eval_tour(&mut used, m.budget, tour)?;
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
        )?; // tud obem o "?" aqui..

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
            // &mut used,
            // m.budget,
            &mut best_tour,
            &mut best_score,
            &pop_[..],
        );

        let (parents, children) = pop_.split_at_mut(m.cut_size);
        for child in children.iter_mut() {
            let parents = (m.match_maker)(parents);
            let new_child = (m.mate) (parents);
            let new_child = (m.mutator)(&mut m.rng, new_child);
            let mut visited = BitSet::with_capacity(m.p.len);
            visited.insert(0); visited.insert(1);
            new_child.iter().for_each(|&p| {
                visited.insert(p as usize);
            });
            
            let Some(status) = m.p.eval_tour(
                &mut used,
                m.budget,
                &new_child,
            ) else { return Ok(best_tour); };
            let fitness = fit(status);

            let ctx = LocalSearchCtx {
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

            match (m.searcher)(ctx) {
                Err(_) => return Ok(best_tour),
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
    // let status = p.eval_route(tour);

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

use crate::consume_credit;
fn updates_best(
    p: &Problem,
    // used: &mut usize,
    // budget: usize,
    best_tour: &mut Vec<u8>,
    best_score: &mut u32,
    pop: &[(Vec<u8>, f32, TourStatus)] // assumed to be sorted
) {
    for (tour, _, status) in pop {
        // consume_credit!(used, budget);
        // let status = p.eval_route(tour);
        if status.score <= *best_score { return () }
        if status.cost > p.tmax { continue }

        *best_score = status.score;
        *best_tour = tour.clone();
        return ()
        // return Ok(());
    } 

    // Ok(())
}

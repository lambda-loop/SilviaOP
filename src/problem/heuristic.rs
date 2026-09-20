pub mod dispersion;
pub mod experiment;
pub mod strategy;

use crate::problem::Problem;

use super::route::RouteStatus;

use std::collections::HashSet;

use strategy::{
    Candidate,
    Selector,
};

const NTIMES: usize = 5_000;

const DIVERSITY_SAMPLE: usize = 1_000;
const N_CLOSE: usize = 5;

// 0 = start node
// 1 = end node
// 2.. = visitable nodes
const START_NODE: u8 = 0;
const END_NODE: u8 = 1;


// -----------------------------------------------------------------------------
// Basic construction
// -----------------------------------------------------------------------------

pub fn single_best_insertion_by(
    p: &Problem,
    r: &mut Vec<u8>,
    unvisited: &mut HashSet<u8>,
    selector: &Selector,
) -> bool {
    let mut candidates = Vec::new();

    let mut new_r =
        Vec::with_capacity(r.len() + 1);

    for k in 0..=r.len() {
        for &u in unvisited.iter() {
            new_r.clear();
            new_r.extend_from_slice(r);
            new_r.insert(k, u);

            let status =
                p.eval_route(&new_r);

            if status.total_consume > p.tmax {
                continue;
            }


            // ------------------------------------------------------------
            // Marginal score
            // ------------------------------------------------------------

            let score_gain =
                p.scores[u as usize] as f64;


            // ------------------------------------------------------------
            // Marginal insertion cost
            //
            // Before:
            //
            //     prev ------> next
            //
            // After:
            //
            //     prev -> u -> next
            //
            // Therefore:
            //
            // ΔC =
            //     c(prev, u)
            //   + c(u, next)
            //   - c(prev, next)
            //
            // Works even when r is empty:
            //
            //     prev = START_NODE
            //     next = END_NODE
            // ------------------------------------------------------------

            let prev =
                if k == 0 {
                    START_NODE as usize
                } else {
                    r[k - 1] as usize
                };


            let next =
                if k == r.len() {
                    END_NODE as usize
                } else {
                    r[k] as usize
                };


            let u_idx =
                u as usize;


            let cost_increase =
                p.costs[(prev, u_idx)] as f64
                + p.costs[(u_idx, next)] as f64
                - p.costs[(prev, next)] as f64;


            candidates.push(Candidate {
                u,
                k,
                status,
                score_gain,
                cost_increase,
            });
        }
    }


    if candidates.is_empty() {
        return false;
    }


    let i =
        selector.select(&candidates);


    let u =
        candidates[i].u;

    let k =
        candidates[i].k;


    r.insert(k, u);

    unvisited.remove(&u);


    true
}

pub fn run_single(
    p: &Problem,
    selector: &Selector,
) -> Vec<u8> {
    let mut r =
        Vec::with_capacity(p.len);

    let mut unvisited =
        HashSet::new();

    for u in 2..p.len {
        unvisited.insert(u as u8);
    }

    while single_best_insertion_by(
        p,
        &mut r,
        &mut unvisited,
        selector,
    ) {}

    r
}


// -----------------------------------------------------------------------------
// Single experiment
// -----------------------------------------------------------------------------

use experiment::Result as ER;

pub fn run_single_experiment(
    p: &Problem,
    problem_name: String,
    selector: &Selector,
    method: String,
) -> ER {
    let r =
        run_single(p, selector);

    let sts =
        p.eval_route(&r);

    ER {
        problem_name,
        method,
        cost: sts.total_consume,
        score: sts.total_score,
        route: r,
    }
}


// -----------------------------------------------------------------------------
// Experiment specifications
// -----------------------------------------------------------------------------
//
// RANDOMIZED-ONLY BATTERY.
//
// The deterministic marginal_* and total_* methods were already evaluated in
// the previous experiment. This battery isolates the stochastic mechanisms:
//
//   1. true uniform random feasible insertion;
//   2. random pairwise comparison;
//   3. criterion resampling at every pairwise comparison;
//   4. criterion resampling once per construction step;
//   5. the selected three-strategy random family;
//   6. quality-weighted roulette over the selected strategies;
//   7. every pair/triple/full mixed combination of the selected strategies
//      plus the pure random comparator.
//

#[derive(Clone)]
pub struct ExperimentSpec {
    pub name: String,
    pub selector: Selector,
}


pub fn experiment_specs() -> Vec<ExperimentSpec> {
    use strategy::{
        marginal_envy,
        marginal_wise,
        mixed_random_marginal_comparison,
        mixed_random_total_comparison,
        random_comparison,
        total_envy,
        MARGINAL_ALL,
        RANDOM_COMBINATION_POOL,
        SELECTED_POOL,
        SELECTED_WEIGHTED_POOL,
        TOTAL_ALL,
    };

    let mut specs =
        Vec::new();


    // =========================================================================
    // 1. Pure-random baselines
    // =========================================================================

    // TRUE random construction:
    // uniformly sample one feasible (vertex, position) insertion and repeat
    // until no feasible insertion remains.
    specs.push(
        ExperimentSpec {
            name:
                "random_feasible_insertion".into(),

            selector:
                Selector::Random,
        }
    );


    // Random A-vs-B comparator.
    //
    // This is deliberately NOT the same distribution as Selector::Random:
    // candidate selection occurs through a sequential random tournament.
    specs.push(
        ExperimentSpec {
            name:
                "random_pairwise".into(),

            selector:
                Selector::By(
                    random_comparison
                ),
        }
    );


    // =========================================================================
    // 2. Criterion changes at EVERY pairwise comparison
    // =========================================================================

    specs.push(
        ExperimentSpec {
            name:
                "mixed_random_marginal_all".into(),

            selector:
                Selector::By(
                    mixed_random_marginal_comparison
                ),
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "mixed_random_total_all".into(),

            selector:
                Selector::By(
                    mixed_random_total_comparison
                ),
        }
    );


    // =========================================================================
    // 3. One uniformly random criterion per construction step
    // =========================================================================

    specs.push(
        ExperimentSpec {
            name:
                "random_step_marginal_all".into(),

            selector:
                Selector::RandomCriterion(
                    MARGINAL_ALL
                ),
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "random_step_marginal_all_random_tie".into(),

            selector:
                Selector::RandomCriterionRandomTie(
                    MARGINAL_ALL
                ),
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "random_step_total_all".into(),

            selector:
                Selector::RandomCriterion(
                    TOTAL_ALL
                ),
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "random_step_total_all_random_tie".into(),

            selector:
                Selector::RandomCriterionRandomTie(
                    TOTAL_ALL
                ),
        }
    );


    // =========================================================================
    // 4. Final selected random family
    //
    // Uniform pool:
    //
    //     marginal_envy
    //     total_envy
    //     marginal_wise
    // =========================================================================

    specs.push(
        ExperimentSpec {
            name:
                "selected_random".into(),

            selector:
                Selector::RandomCriterion(
                    SELECTED_POOL
                ),
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "selected_random_random_tie".into(),

            selector:
                Selector::RandomCriterionRandomTie(
                    SELECTED_POOL
                ),
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "selected_random_tie_marginal_envy".into(),

            selector:
                Selector::RandomCriterionWithTie {
                    strategies:
                        SELECTED_POOL,

                    tie_breaker:
                        marginal_envy,
                },
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "selected_random_tie_total_envy".into(),

            selector:
                Selector::RandomCriterionWithTie {
                    strategies:
                        SELECTED_POOL,

                    tie_breaker:
                        total_envy,
                },
        }
    );


    specs.push(
        ExperimentSpec {
            name:
                "selected_random_tie_marginal_wise".into(),

            selector:
                Selector::RandomCriterionWithTie {
                    strategies:
                        SELECTED_POOL,

                    tie_breaker:
                        marginal_wise,
                },
        }
    );


    // =========================================================================
    // 5. Quality-weighted roulette
    //
    // Weights are proportional to the previous aggregate mean relative median
    // quality. The selected criterion is sampled ONCE per construction step.
    //
    // Approximate raw weights currently stored in strategy.rs:
    //
    //     total_envy      0.971
    //     marginal_envy   0.963
    //     marginal_wise   0.948
    //
    // They are intentionally NOT manually normalized: the roulette selector
    // normalizes by their sum.
    // =========================================================================

    specs.push(
        ExperimentSpec {
            name:
                "selected_weighted_random".into(),

            selector:
                Selector::WeightedRandomCriterion(
                    SELECTED_WEIGHTED_POOL
                ),
        }
    );


    // Same weighted roulette, but exact ties under the sampled criterion are
    // resolved uniformly at random.
    specs.push(
        ExperimentSpec {
            name:
                "selected_weighted_random_random_tie".into(),

            selector:
                Selector::WeightedRandomCriterionRandomTie(
                    SELECTED_WEIGHTED_POOL
                ),
        }
    );


    // =========================================================================
    // 6. Exhaustive mixed combinations
    //
    // RANDOM_COMBINATION_POOL contains:
    //
    //     marginal_envy
    //     total_envy
    //     marginal_wise
    //     random_comparison
    //
    // Every subset of size >= 2 is tested. With four strategies this gives:
    //
    //     C(4,2) + C(4,3) + C(4,4) = 6 + 4 + 1 = 11 methods.
    //
    // A NEW comparator from the chosen subset is sampled for EVERY pairwise
    // candidate comparison.
    // =========================================================================

    let n =
        RANDOM_COMBINATION_POOL.len();

    assert!(
        n < usize::BITS as usize,
        "too many strategies for bit-mask combination generation"
    );

    let end =
        1usize << n;

    for mask in 1usize..end {
        let size =
            mask.count_ones() as usize;

        if size < 2 {
            continue;
        }

        let names:
            Vec<&'static str> =
            RANDOM_COMBINATION_POOL
                .iter()
                .enumerate()
                .filter_map(
                    |(index, strategy)| {
                        if mask
                            & (1usize << index)
                            != 0
                        {
                            Some(strategy.name)
                        } else {
                            None
                        }
                    }
                )
                .collect();

        specs.push(
            ExperimentSpec {
                name:
                    format!(
                        "mixed_each_comparison_{}",
                        names.join("_")
                    ),

                selector:
                    Selector::MixedCriterionMask(
                        mask
                    ),
            }
        );
    }


    specs
}


// -----------------------------------------------------------------------------
// Statistical experiment
// -----------------------------------------------------------------------------

pub fn single_results(
    p: &Problem,
    p_name: &str,
    selector: &Selector,
    selector_name: &str,
) -> Vec<EResult> {
    let mut scores =
        Vec::with_capacity(NTIMES);

    // We do NOT need to store every produced route.
    // Only a sample is kept for structural diversity analysis.
    let mut routes =
        Vec::with_capacity(
            DIVERSITY_SAMPLE.min(NTIMES),
        );


    for run_idx in 0..NTIMES {
        let mut r =
            Vec::with_capacity(p.len);

        let mut unvisited =
            HashSet::new();

        for u in 2..p.len {
            unvisited.insert(u as u8);
        }


        while single_best_insertion_by(
            p,
            &mut r,
            &mut unvisited,
            selector,
        ) {}


        let RouteStatus {
            total_score,
            ..
        } = p.eval_route(&r);

        scores.push(total_score);


        // -------------------------------------------------------------
        // Uniform random sample for structural diversity
        // -------------------------------------------------------------
        //
        // Reservoir sampling keeps exactly DIVERSITY_SAMPLE routes
        // (when NTIMES >= DIVERSITY_SAMPLE) while giving every one of
        // the NTIMES generated routes the same probability of belonging
        // to the final sample.
        //
        // This avoids systematically using only the first generated
        // routes without having to store all NTIMES routes in memory.
        if DIVERSITY_SAMPLE > 0 {
            if routes.len() < DIVERSITY_SAMPLE {
                routes.push(r);
            } else {
                let j =
                    rand::random_range(
                        0..=run_idx
                    );

                if j < DIVERSITY_SAMPLE {
                    routes[j] = r;
                }
            }
        }
    }


    // -------------------------------------------------------------------------
    // Objective-value statistics
    // -------------------------------------------------------------------------

    scores.sort_unstable();

    let n =
        scores.len();


    let worst =
        scores[0] as f64;

    let best =
        scores[n - 1] as f64;


    let median =
        if n % 2 == 0 {
            (
                scores[n / 2 - 1] as f64
                    + scores[n / 2] as f64
            ) / 2.0
        } else {
            scores[n / 2] as f64
        };


    let q1 =
        scores[n / 4] as f64;

    let q3 =
        scores[3 * n / 4] as f64;


    let average =
        scores
            .iter()
            .map(|&x| x as f64)
            .sum::<f64>()
            / n as f64;


    // -------------------------------------------------------------------------
    // Structural diversity between routes
    // -------------------------------------------------------------------------

    let route_diversity =
        dispersion::average_pairwise_distance(
            &routes,
            START_NODE,
            END_NODE,
        );


    let local_route_diversity =
        dispersion::average_knn_distance(
            &routes,
            N_CLOSE,
            START_NODE,
            END_NODE,
        );


    // -------------------------------------------------------------------------
    // Results
    // -------------------------------------------------------------------------

    [
        (
            InfoType::Median,
            median,
        ),

        (
            InfoType::Q1,
            q1,
        ),

        (
            InfoType::Q3,
            q3,
        ),

        (
            InfoType::Best,
            best,
        ),

        (
            InfoType::Worst,
            worst,
        ),

        (
            InfoType::Average,
            average,
        ),

        (
            InfoType::RouteDiversity,
            route_diversity,
        ),

        (
            InfoType::LocalRouteDiversity,
            local_route_diversity,
        ),
    ]
    .into_iter()
    .map(
        |(info_t, info)| {
            EResult {
                p_name:
                    p_name.to_string(),

                s_name:
                    selector_name.to_string(),

                info_t,

                info,
            }
        }
    )
    .collect()
}


// -----------------------------------------------------------------------------
// Run every configured method over one problem
// -----------------------------------------------------------------------------

pub fn multi_results(
    p: &Problem,
    p_name: &str,
    specs: &[ExperimentSpec],
) -> Vec<EResult> {
    let mut results =
        Vec::new();

    for spec in specs {
        results.extend(
            single_results(
                p,
                p_name,
                &spec.selector,
                &spec.name,
            )
        );
    }

    println!("done: {p_name}");

    results
}


// -----------------------------------------------------------------------------
// CSV result
// -----------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq
)]
pub enum InfoType {
    Median,
    Q1,
    Q3,

    Best,
    Worst,
    Average,

    // Mean distance over every sampled pair.
    RouteDiversity,

    // Mean distance to the N_CLOSE nearest routes.
    LocalRouteDiversity,
}


#[derive(
    Debug,
    Clone,
    PartialEq
)]
pub struct EResult {
    pub p_name: String,
    pub s_name: String,
    pub info_t: InfoType,
    pub info: f64,
}


impl EResult {
    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{:?},{}",
            self.p_name,
            self.s_name,
            self.info_t,
            self.info,
        )
    }


    pub fn header() -> &'static str {
        "p_name,s_name,info_type,info"
    }
}

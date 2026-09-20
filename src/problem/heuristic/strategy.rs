use crate::problem::RouteStatus;
use std::cmp::Ordering;


// ============================================================================
// Candidate
// ============================================================================

/// A feasible insertion candidate.
///
/// `status` describes the complete route AFTER the insertion.
///
/// `score_gain` and `cost_increase` describe only the marginal
/// effect caused by this insertion:
///
///     score_gain    = S(R') - S(R)
///     cost_increase = C(R') - C(R)
///
pub struct Candidate {
    pub u: u8,
    pub k: usize,

    pub status: RouteStatus,

    pub score_gain: f64,
    pub cost_increase: f64,
}


// ============================================================================
// Strategy types
// ============================================================================

/// Ordering::Less means that `l` is preferred over `r`.
pub type StrategyFn =
    fn(&Candidate, &Candidate) -> Ordering;

type MetricFn =
    fn(&Candidate) -> f64;


// ============================================================================
// Strategy families
// ============================================================================

/// New/recommended strategies.
///
/// These compare the MARGINAL effect of an insertion.
pub const MARGINAL_ALL: &[StrategyFn] = &[
    marginal_greedy,
    marginal_lazy,
    marginal_smart,
    marginal_wise,
    marginal_envy,
];

pub const MARGINAL_BEST: &[StrategyFn] = &[
    marginal_smart,
    marginal_wise,
    marginal_envy,
];


/// Original strategies.
///
/// These compare the TOTAL status of the candidate route,
/// reproducing the behavior used in the previous experiments.
pub const TOTAL_ALL: &[StrategyFn] = &[
    total_greedy,
    total_lazy,
    total_smart,
    total_wise,
    total_envy,
];

pub const TOTAL_BEST: &[StrategyFn] = &[
    total_smart,
    total_wise,
    total_envy,
];

/// Strategies selected for the final randomized experiments.
pub const SELECTED_POOL: &[StrategyFn] = &[
    marginal_envy,
    total_envy,
    marginal_wise,
];


/// Default strategy family.
///
/// From now on, unqualified strategy names refer to
/// the marginal formulation.
pub const ALL: &[StrategyFn] =
    MARGINAL_ALL;

pub const BEST: &[StrategyFn] =
    MARGINAL_BEST;


pub const METHODS: [&str; 5] = [
    "greedy",
    "lazy",
    "smart",
    "wise",
    "envy",
];


pub const MARGINAL_METHODS: [&str; 5] = [
    "marginal_greedy",
    "marginal_lazy",
    "marginal_smart",
    "marginal_wise",
    "marginal_envy",
];


pub const TOTAL_METHODS: [&str; 5] = [
    "total_greedy",
    "total_lazy",
    "total_smart",
    "total_wise",
    "total_envy",
];


// ============================================================================
// Secondary selection
// ============================================================================

#[derive(Clone, Copy)]
pub enum Secondary {
    /// Apply a comparison rule.
    By(StrategyFn),

    /// Apply the rule, but solve exact ties randomly.
    ByRandomTie(StrategyFn),

    /// Randomly choose one candidate from the subset.
    Random,
}


// ============================================================================
// Selector
// ============================================================================

#[derive(Clone, Copy)]
pub enum Selector {
    /// Standard strategy.
    By(StrategyFn),

    /// Standard strategy with explicit random tie breaking.
    ByRandomTie(StrategyFn),

    /// Uniformly select one feasible insertion.
    Random,

    /// Randomly choose one criterion at each construction step
    /// and use it consistently throughout that step.
    RandomCriterion(
        &'static [StrategyFn]
    ),


    /// Randomly chooses one primary criterion for the
    /// current construction step.
    ///
    /// Exact ties under that criterion are solved randomly.
    RandomCriterionRandomTie(
        &'static [StrategyFn]
    ),

    /// Randomly chooses one primary criterion for the
    /// current construction step.
    ///
    /// If two candidates tie under the primary criterion,
    /// `tie_breaker` is used.
    ///
    /// If they are still tied, the final tie is random.
    RandomCriterionWithTie {
        strategies: &'static [StrategyFn],
        tie_breaker: StrategyFn,
    },

    /// Keep the best fraction according to `primary`,
    /// then choose inside that subset using `secondary`.
    TopFraction {
        primary: StrategyFn,
        secondary: Secondary,
        fraction: f64,
    },

    /// Marginal formulation:
    ///
    /// compares ΔS and ΔC.
    DecisiveScoreCost,

    /// Old/total formulation:
    ///
    /// compares S(R') and C(R').
    DecisiveScoreCostTotal,

    /// Marginal formulation:
    ///
    /// compares the relative dispersion of ΔS and ΔC.
    MostDiscriminatingScoreCost,

    /// Old/total formulation:
    ///
    /// compares the relative dispersion of S(R') and C(R').
    MostDiscriminatingScoreCostTotal,
}


// ============================================================================
// Utility functions
// ============================================================================

fn ratio(
    score: f64,
    cost: f64,
    score_power: i32,
    cost_power: i32,
) -> f64 {
    /*
     * In our OP instances, insertion costs are expected to be
     * non-negative.
     *
     * If a positive-score insertion has effectively zero cost,
     * it is maximally attractive.
     */
    if cost <= f64::EPSILON {
        if score > 0.0 {
            return f64::INFINITY;
        }

        return 0.0;
    }

    score.powi(score_power)
        / cost.powi(cost_power)
}


fn relative_gap(
    a: f64,
    b: f64,
) -> f64 {
    let denominator =
        a.abs()
            .max(b.abs())
            .max(f64::EPSILON);

    (a - b).abs()
        / denominator
}


fn relative_std(
    values: &[f64],
) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mean =
        values
            .iter()
            .sum::<f64>()
            / values.len() as f64;

    if mean.abs() <= f64::EPSILON {
        return 0.0;
    }

    let variance =
        values
            .iter()
            .map(
                |x| {
                    (x - mean)
                        .powi(2)
                }
            )
            .sum::<f64>()
            / values.len() as f64;

    variance.sqrt()
        / mean.abs()
}


// ============================================================================
// Metric extraction
// ============================================================================

fn marginal_score(
    c: &Candidate,
) -> f64 {
    c.score_gain
}


fn marginal_cost(
    c: &Candidate,
) -> f64 {
    c.cost_increase
}


fn total_score(
    c: &Candidate,
) -> f64 {
    c.status.total_score as f64
}


fn total_cost(
    c: &Candidate,
) -> f64 {
    c.status.total_consume as f64
}


// ============================================================================
// MARGINAL elementary strategies
// ============================================================================

/// Maximize marginal score:
///
///     max ΔS
///
/// Notice that this produces the same ordering as maximizing
/// total score during a single insertion step.
pub fn marginal_greedy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    r.score_gain
        .total_cmp(
            &l.score_gain
        )
}


/// Minimize marginal insertion cost:
///
///     min ΔC
///
/// This produces the same ordering as minimizing total
/// candidate cost during a single insertion step.
pub fn marginal_lazy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    l.cost_increase
        .total_cmp(
            &r.cost_increase
        )
}


/// Marginal reward density:
///
///           ΔS
///     max  ----
///           ΔC
///
pub fn marginal_smart(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let l_value =
        ratio(
            l.score_gain,
            l.cost_increase,
            1,
            1,
        );

    let r_value =
        ratio(
            r.score_gain,
            r.cost_increase,
            1,
            1,
        );

    r_value.total_cmp(
        &l_value
    )
}


/// Cost-sensitive marginal reward density:
///
///            ΔS
///     max  ------
///           ΔC²
///
pub fn marginal_wise(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let l_value =
        ratio(
            l.score_gain,
            l.cost_increase,
            1,
            2,
        );

    let r_value =
        ratio(
            r.score_gain,
            r.cost_increase,
            1,
            2,
        );

    r_value.total_cmp(
        &l_value
    )
}


/// Score-sensitive marginal reward density:
///
///           ΔS²
///     max  -----
///           ΔC
///
pub fn marginal_envy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let l_value =
        ratio(
            l.score_gain,
            l.cost_increase,
            2,
            1,
        );

    let r_value =
        ratio(
            r.score_gain,
            r.cost_increase,
            2,
            1,
        );

    r_value.total_cmp(
        &l_value
    )
}


// ============================================================================
// TOTAL / LEGACY elementary strategies
// ============================================================================

/// Original greedy criterion:
///
///     max S(R')
///
pub fn total_greedy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    r.status
        .total_score
        .cmp(
            &l.status.total_score
        )
}


/// Original lazy criterion:
///
///     min C(R')
///
pub fn total_lazy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    l.status
        .total_consume
        .total_cmp(
            &r.status.total_consume
        )
}


/// Original smart criterion:
///
///           S(R')
///     max  ------
///           C(R')
///
pub fn total_smart(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let l_value =
        ratio(
            total_score(l),
            total_cost(l),
            1,
            1,
        );

    let r_value =
        ratio(
            total_score(r),
            total_cost(r),
            1,
            1,
        );

    r_value.total_cmp(
        &l_value
    )
}


/// Original wise criterion:
///
///           S(R')
///     max  -------
///           C(R')²
///
pub fn total_wise(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let l_value =
        ratio(
            total_score(l),
            total_cost(l),
            1,
            2,
        );

    let r_value =
        ratio(
            total_score(r),
            total_cost(r),
            1,
            2,
        );

    r_value.total_cmp(
        &l_value
    )
}


/// Original envy criterion:
///
///           S(R')²
///     max  -------
///           C(R')
///
pub fn total_envy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let l_value =
        ratio(
            total_score(l),
            total_cost(l),
            2,
            1,
        );

    let r_value =
        ratio(
            total_score(r),
            total_cost(r),
            2,
            1,
        );

    r_value.total_cmp(
        &l_value
    )
}


// ============================================================================
// Default names
// ============================================================================
//
// These preserve the nice short names throughout the rest of the project.
//
// IMPORTANT:
// The default now means MARGINAL.
//

pub fn greedy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    marginal_greedy(l, r)
}


pub fn lazy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    marginal_lazy(l, r)
}


pub fn smart(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    marginal_smart(l, r)
}


pub fn wise(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    marginal_wise(l, r)
}


pub fn envy(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    marginal_envy(l, r)
}


// ============================================================================
// Random comparison strategies
// ============================================================================

/// Original peculiar random behavior, but using the NEW
/// marginal strategy family.
///
/// A new criterion is selected for EVERY comparison.
///
/// This is intentionally non-transitive and must NOT be used
/// as a sorting comparator.
pub fn mixed_random_comparison(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    mixed_random_marginal_comparison(
        l,
        r,
    )
}


/// Mixed random using marginal strategies.
pub fn mixed_random_marginal_comparison(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let f =
        MARGINAL_ALL[
            rand::random_range(
                0..MARGINAL_ALL.len()
            )
        ];

    f(l, r)
}


/// Exact old mixed-random behavior,
/// using total candidate-route measures.
pub fn mixed_random_total_comparison(
    l: &Candidate,
    r: &Candidate,
) -> Ordering {
    let f =
        TOTAL_ALL[
            rand::random_range(
                0..TOTAL_ALL.len()
            )
        ];

    f(l, r)
}


// ============================================================================
// Candidate comparison helpers
// ============================================================================

fn best_by(
    candidates: &[Candidate],
    indices: &[usize],
    cmp: StrategyFn,
    random_ties: bool,
) -> usize {
    let mut best =
        indices[0];

    let mut ties =
        1usize;

    for &i in &indices[1..] {
        match cmp(
            &candidates[i],
            &candidates[best],
        ) {
            Ordering::Less => {
                best = i;
                ties = 1;
            }

            Ordering::Equal
                if random_ties =>
            {
                ties += 1;

                if rand::random_range(
                    0..ties
                ) == 0
                {
                    best = i;
                }
            }

            _ => {}
        }
    }

    best
}


fn best_all(
    candidates: &[Candidate],
    cmp: StrategyFn,
    random_ties: bool,
) -> usize {
    let indices: Vec<_> =
        (0..candidates.len())
            .collect();

    best_by(
        candidates,
        &indices,
        cmp,
        random_ties,
    )
}


// ============================================================================
// Generic score/cost selectors
// ============================================================================

fn decisive_score_cost(
    candidates: &[Candidate],
    score_cmp: StrategyFn,
    cost_cmp: StrategyFn,
    score_value: MetricFn,
    cost_value: MetricFn,
) -> usize {
    if candidates.len() == 1 {
        return 0;
    }

    let mut by_score:
        Vec<_> =
        (0..candidates.len())
            .collect();

    let mut by_cost =
        by_score.clone();


    by_score.sort_by(
        |&l, &r| {
            score_cmp(
                &candidates[l],
                &candidates[r],
            )
        }
    );


    by_cost.sort_by(
        |&l, &r| {
            cost_cmp(
                &candidates[l],
                &candidates[r],
            )
        }
    );


    let score_best =
        by_score[0];

    let score_second =
        by_score[1];

    let cost_best =
        by_cost[0];

    let cost_second =
        by_cost[1];


    let score_gap =
        relative_gap(
            score_value(
                &candidates[score_best]
            ),
            score_value(
                &candidates[score_second]
            ),
        );


    let cost_gap =
        relative_gap(
            cost_value(
                &candidates[cost_best]
            ),
            cost_value(
                &candidates[cost_second]
            ),
        );


    if score_gap > cost_gap {
        score_best
    } else if cost_gap > score_gap {
        cost_best
    } else if score_best == cost_best {
        score_best
    } else if rand::random::<bool>() {
        score_best
    } else {
        cost_best
    }
}


fn most_discriminating_score_cost(
    candidates: &[Candidate],
    score_cmp: StrategyFn,
    cost_cmp: StrategyFn,
    score_value: MetricFn,
    cost_value: MetricFn,
) -> usize {
    let scores: Vec<f64> =
        candidates
            .iter()
            .map(score_value)
            .collect();

    let costs: Vec<f64> =
        candidates
            .iter()
            .map(cost_value)
            .collect();


    let score_dispersion =
        relative_std(
            &scores
        );

    let cost_dispersion =
        relative_std(
            &costs
        );


    if score_dispersion
        > cost_dispersion
    {
        best_all(
            candidates,
            score_cmp,
            false,
        )
    } else if cost_dispersion
        > score_dispersion
    {
        best_all(
            candidates,
            cost_cmp,
            false,
        )
    } else if rand::random::<bool>() {
        best_all(
            candidates,
            score_cmp,
            false,
        )
    } else {
        best_all(
            candidates,
            cost_cmp,
            false,
        )
    }
}


// ============================================================================
// Selector implementation
// ============================================================================

impl Selector {
    pub fn select(
        &self,
        candidates: &[Candidate],
    ) -> usize {
        assert!(
            !candidates.is_empty()
        );

        match *self {
            // -----------------------------------------------------------------
            // Standard strategy
            // -----------------------------------------------------------------

            Selector::By(cmp) => {
                best_all(
                    candidates,
                    cmp,
                    false,
                )
            }


            // -----------------------------------------------------------------
            // Strategy + explicit random tie-breaking
            // -----------------------------------------------------------------

            Selector::ByRandomTie(
                cmp
            ) => {
                best_all(
                    candidates,
                    cmp,
                    true,
                )
            }


            // -----------------------------------------------------------------
            // Truly random feasible insertion
            // -----------------------------------------------------------------

            Selector::Random => {
                rand::random_range(
                    0..candidates.len()
                )
            }


            // -----------------------------------------------------------------
            // Random criterion per construction step
            // -----------------------------------------------------------------

            Selector::RandomCriterion(
                strategies
            ) => {
                assert!(
                    !strategies.is_empty()
                );

                let cmp =
                    strategies[
                        rand::random_range(
                            0..strategies.len()
                        )
                    ];

                best_all(
                    candidates,
                    cmp,
                    false,
                )
            }


            // -----------------------------------------------------------------
            // Random criterion + explicit random tie-breaking
            // -----------------------------------------------------------------

            Selector::RandomCriterionRandomTie(
                strategies
            ) => {
                assert!(
                    !strategies.is_empty()
                );

                let cmp =
                    strategies[
                        rand::random_range(
                            0..strategies.len()
                        )
                    ];

                best_all(
                    candidates,
                    cmp,
                    true,
                )
            }


            // -----------------------------------------------------------------
            // Random criterion + secondary criterion for ties
            // -----------------------------------------------------------------

            Selector::RandomCriterionWithTie {
                strategies,
                tie_breaker,
            } => {
                assert!(
                    !strategies.is_empty()
                );

                let primary =
                    strategies[
                        rand::random_range(
                            0..strategies.len()
                        )
                    ];

                best_all_then(
                    candidates,
                    primary,
                    tie_breaker,
                )
            }


            // -----------------------------------------------------------------
            // Top fraction
            // -----------------------------------------------------------------

            Selector::TopFraction {
                primary,
                secondary,
                fraction,
            } => {
                let fraction =
                    fraction.clamp(
                        0.0,
                        1.0,
                    );


                let mut indices:
                    Vec<_> =
                    (0..candidates.len())
                        .collect();


                /*
                 * `primary` MUST define a consistent ordering.
                 *
                 * Do not use either mixed_random_* comparator here.
                 */
                indices.sort_by(
                    |&l, &r| {
                        primary(
                            &candidates[l],
                            &candidates[r],
                        )
                    }
                );


                let keep =
                    (
                        candidates.len()
                            as f64
                            * fraction
                    )
                    .ceil()
                        as usize;


                let keep =
                    keep.clamp(
                        1,
                        candidates.len(),
                    );


                let top =
                    &indices[..keep];


                match secondary {
                    Secondary::By(
                        cmp
                    ) => {
                        best_by(
                            candidates,
                            top,
                            cmp,
                            false,
                        )
                    }


                    Secondary::ByRandomTie(
                        cmp
                    ) => {
                        best_by(
                            candidates,
                            top,
                            cmp,
                            true,
                        )
                    }


                    Secondary::Random => {
                        top[
                            rand::random_range(
                                0..top.len()
                            )
                        ]
                    }
                }
            }


            // -----------------------------------------------------------------
            // Marginal decisive score/cost
            // -----------------------------------------------------------------

            Selector::DecisiveScoreCost => {
                decisive_score_cost(
                    candidates,

                    marginal_greedy,
                    marginal_lazy,

                    marginal_score,
                    marginal_cost,
                )
            }


            // -----------------------------------------------------------------
            // Total / old decisive score/cost
            // -----------------------------------------------------------------

            Selector::DecisiveScoreCostTotal => {
                decisive_score_cost(
                    candidates,

                    total_greedy,
                    total_lazy,

                    total_score,
                    total_cost,
                )
            }


            // -----------------------------------------------------------------
            // Marginal most-discriminating score/cost
            // -----------------------------------------------------------------

            Selector::MostDiscriminatingScoreCost => {
                most_discriminating_score_cost(
                    candidates,

                    marginal_greedy,
                    marginal_lazy,

                    marginal_score,
                    marginal_cost,
                )
            }


            // -----------------------------------------------------------------
            // Total / old most-discriminating score/cost
            // -----------------------------------------------------------------

            Selector::MostDiscriminatingScoreCostTotal => {
                most_discriminating_score_cost(
                    candidates,

                    total_greedy,
                    total_lazy,

                    total_score,
                    total_cost,
                )
            }
        }
    }
}

fn best_by_then(
    candidates: &[Candidate],
    indices: &[usize],

    primary: StrategyFn,
    tie_breaker: StrategyFn,
) -> usize {
    let mut best =
        indices[0];

    let mut ties =
        1usize;


    for &i in &indices[1..] {
        match primary(
            &candidates[i],
            &candidates[best],
        ) {
            Ordering::Less => {
                best = i;
                ties = 1;
            }


            Ordering::Equal => {
                match tie_breaker(
                    &candidates[i],
                    &candidates[best],
                ) {
                    Ordering::Less => {
                        best = i;
                        ties = 1;
                    }


                    Ordering::Equal => {
                        // Still tied after the secondary
                        // criterion: uniform random tie.
                        ties += 1;

                        if rand::random_range(
                            0..ties
                        ) == 0
                        {
                            best = i;
                        }
                    }


                    Ordering::Greater => {}
                }
            }


            Ordering::Greater => {}
        }
    }


    best
}


fn best_all_then(
    candidates: &[Candidate],

    primary: StrategyFn,
    tie_breaker: StrategyFn,
) -> usize {
    let indices: Vec<_> =
        (0..candidates.len())
            .collect();


    best_by_then(
        candidates,
        &indices,
        primary,
        tie_breaker,
    )
}

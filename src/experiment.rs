//

use crate::config;
use crate::problem::Problem;
use crate::problem::Tour;

// Represents a bunch of experiments for a single prblem using a single method!
#[derive(Debug, Clone)]
pub struct Result {
    pub tour: Tour,
    pub time: std::time::Duration,
    pub iters: u64,
}

#[derive(Debug)]
pub struct Conclusion {
    pub problem_name   : String,
    pub problem_size   : usize,
    pub problem_budget : f64,

    pub method_name    : String,

    pub worst_score        : f64,
    pub best_score         : f64,

    pub mean_score         : f64,
    pub median_score       : f64,
    pub score_variance     : f64,
    pub standard_deviation : f64,

    pub worst_tour: Tour,
    pub best_tour : Tour,

    // in millis 
    pub mean_time : f64,
    pub mean_iters: f64,

    pub num_shots: usize,
    pub credits: u64,
    pub extra_info: String,

}

impl Conclusion {
    pub fn new(
        rs: Vec<Result>,
        p: &Problem,
        problem_name: String,
        method_name: String,
        num_shots: usize,
        credits: u64,
        extra_info: String,
    ) -> Self {
        let mut rs: Vec<_> = rs.into_iter().map(|r| {
            let r_score = p.eval_route(&r.tour).total_score;
            (r, r_score as f64)
        }).collect();

        rs.sort_unstable_by(|(_, l), (_, r)| l.total_cmp(r));
        let (worst_r, worst_score) = rs[0].clone();
        let (best_r, best_score)  = rs.last().expect("result values cant be empty");

        let (total_score, total_time) = rs.iter().fold(
            (0., 0.),
            |(acc_score, acc_time), (r, score)|
            (acc_score as f64 + score, r.time.as_secs_f64() * 1000.0 + acc_time)
        );

        let (mean_score, mean_time) = (
            total_score / rs.len() as f64,
            total_time  / rs.len() as f64,
        );

        let median_score = if rs.len() % 2 == 0 {
            let middle = rs.len() / 2;
            (rs[middle - 1].1 + rs[middle].1) / 2.0
        } else {
            rs[rs.len() / 2].1
        };
        
        let score_variance = rs.iter()
            .map(|(_, score)| {
                let diff = score - mean_score;
                diff * diff
            })
            .sum::<f64>()
            / rs.len() as f64;
        
        let standard_deviation = score_variance.sqrt();
        let (best_tour, worst_tour) = (best_r.tour.clone(), worst_r.tour);

        let problem_size = p.len;
        let problem_budget = p.tmax;
        let mean_iters = rs.iter()
            .map(|(r, _)| r.iters as f64)
            .sum::<f64>() / rs.len() as f64;
        
        Self {
            problem_name,
            problem_size,
            problem_budget,

            method_name,
            
            worst_score,
            best_score: *best_score,
            
            mean_score,
            median_score,
            score_variance,
            standard_deviation,
            
            worst_tour,
            best_tour,
            
            mean_time,
            mean_iters,

            num_shots,
            credits,
            extra_info,
        }
    }

    pub fn header() -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            "problem_name",
            "problem_size",
            "problem_budget",

            "method_name",
            "best_score",
            "worst_score",
            "mean_score",
            "median_score",
            "score_variance",
            "standard_deviation",

            "worst_tour",
            "best_tour",

            "mean_time",
            "mean_iters",

            "num_shots",
            "credits",
            "extra_info",
        )
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{:1},{},{:.1},{:.1},{:.2},{:.2},{:.4},{:.4},\"{:?}\",\"{:?}\",{:.2},{:.2},{},{},{}\n",
            self.problem_name,
            self.problem_size,
            self.problem_budget,
            self.method_name,
            self.best_score,
            self.worst_score,
            self.mean_score,
            self.median_score,
            self.score_variance,
            self.standard_deviation,

            self.worst_tour,
            self.best_tour,

            self.mean_time,
            self.mean_iters,

            self.num_shots,
            self.credits,
            self.extra_info,
        )
    }
}

use rayon::prelude::*;
pub trait Method where Self:Sized {
    // type Args;
    fn shot(&mut self, p: &Problem, credits: u64) -> Result;
    fn run (
        &mut self,
        p: &Problem,
        credits: u64,
        num_shots: usize,
        problem_name: &str,
        method_name : &str,
        extra_info: &str,
    ) -> Conclusion {
        let mut shots = Vec::with_capacity(num_shots);
        for _ in 0..num_shots {
            let shot = self.shot(p, credits);
            shots.push(shot);
        }

        Conclusion::new(
            shots,
            p,
            String::from(problem_name), 
            String::from(method_name),
            num_shots,
            credits,
            extra_info.to_string(),
        )
    }

    fn all_ms() -> Vec<(Self, String)>;
    fn run_all_ms_in_p(
        p: &Problem,
        problem_name: &str,
        num_shots_per_method: usize,
    ) { // -> Vec<Conclusion> {
        let ms = Self::all_ms();
        // let mut ers = Vec::new();
        for (mut m, m_name) in ms.into_iter() {
            for credits in config::CREDITS {
                let c = m.run(
                    p,
                    credits,
                    num_shots_per_method,
                    problem_name,
                    &m_name,
                    "", // TODO: problem with no extra
                ).to_csv();
                print!("{c}");
            }
        }
    }

    fn run_all_ms_in_all_ps(
        ps: Vec<(Problem, String)>,
        num_shots_per_method: usize,
    ) { // -> Vec<Conclusion> {
        assert_ne!(num_shots_per_method, 0);
        let ps_len = ps.len();
        
        ps.into_par_iter()
            .for_each(|(p, p_name)| {
                Self::run_all_ms_in_p(
                    &p,
                    &p_name,
                    num_shots_per_method,
                );

            });
    }
}


////////////////// credits system:

#[derive(Debug)]
pub struct OutOfCredits;

#[macro_export]
macro_rules! consume_credit {
    ($counter:expr, $limit:expr) => {
        if *$counter == $limit {
            return Err(OutOfCredits);
        } else {
            *$counter += 1;
        }
    };
}


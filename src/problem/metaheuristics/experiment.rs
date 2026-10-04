//

use crate::problem::Problem;

// Represents a bunch of experiments for a single prblem using a single method!
#[derive(Debug, Clone)]
pub struct Result {
    pub tour: Vec<u8>,
    pub time: std::time::Duration,
}

#[derive(Debug)]
pub struct Conclusion {
    pub problem_name   : String,
    pub problem_size   : usize,
    pub problem_budget : f32,

    pub method_name    : String,

    pub worst_score : f32,
    pub best_score  : f32,

    pub mean_score  : f32,
    pub median_score      : f32,
    pub score_variance    : f32,
    pub standard_deviation : f32,

    pub worst_tour: Vec<u8>,
    pub best_tour : Vec<u8>,

    // in millis 
    pub mean_time: f32,

    pub num_shots: usize,
    pub credits: usize,
    pub extra_info: String,

}

impl Conclusion {
    pub fn new(
        rs: Vec<Result>,
        p: &Problem,
        problem_name: String,
        method_name: String,
        num_shots: usize,
        credits: usize,
        extra_info: String,
    ) -> Self {
        let mut rs: Vec<_> = rs.into_iter().map(|r| {
            let r_score = p.eval_route(&r.tour).total_score;
            (r, r_score as f32)
        }).collect();

        rs.sort_unstable_by(|(_, l), (_, r)| l.total_cmp(r));
        let (worst_r, worst_score) = rs[0].clone();
        let (best_r, best_score)  = rs.last().expect("result values cant be empty");

        let (total_score, total_time) = rs.iter().fold(
            (0., 0.),
            |(acc_score, acc_time), (r, score)|
            (acc_score as f32 + score, r.time.as_secs_f32() * 1000.0 + acc_time)
        );

        let (mean_score, mean_time) = (
            total_score / rs.len() as f32,
            total_time  / rs.len() as f32,
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
            .sum::<f32>()
            / rs.len() as f32;
        
        let standard_deviation = score_variance.sqrt();
        let (best_tour, worst_tour) = (best_r.tour.clone(), worst_r.tour);

        let problem_size = p.len;
        let problem_budget = p.tmax;
        
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

            num_shots,
            credits,
            extra_info,
        }
    }

    pub fn header() -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
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

            "num_shots",
            "credits",
            "extra_info",
        )
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{:1},{},{:.1},{:.1},{:.2},{:.2},{:.4},{:.4},\"{:?}\",\"{:?}\",{:.2},{},{},{}\n",
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

            self.num_shots,
            self.credits,
            self.extra_info,
        )
    }
}



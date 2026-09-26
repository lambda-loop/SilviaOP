//

use crate::problem::Problem;

#[derive(Debug)]
pub struct Result {
    pub problem_name: String,
    pub method_name : String,

    pub worst_score : f32,
    pub best_score  : f32,

    pub mean        : f32,
    pub median      : f32,
    pub variance    : f32,
    pub standard_deviation : f32,

    pub worst_tour: Vec<u8>,
    pub best_tour : Vec<u8>,

    pub mean_time: std::time::Duration,
}

impl Result {
    pub fn header() -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            "problem_name",
            "method_name",
            "best_score",
            "worst_score",
            "mean",
            "median",
            "variance",
            "standard_deviation",

            "worst_tour",
            "best_tour",

            "mean_time",
        )
    }

    pub fn to_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{:?},{:?},{:?}",
            self.problem_name,
            self.method_name,
            self.best_score,
            self.worst_score,
            self.mean,
            self.median,
            self.variance,
            self.standard_deviation,

            self.worst_tour,
            self.best_tour,

            self.mean_time.as_nanos(),
        )
    }
}



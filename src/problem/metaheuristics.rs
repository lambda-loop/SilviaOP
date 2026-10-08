//

// pub mod grasp;
// pub mod multistart;
pub mod experiment;
pub mod memetic;

use experiment::Result as ER;
use experiment::Conclusion;
use crate::problem::Problem;

// pub mod simulated_annealing;

const CREDITS: [usize;3] = [
    100_000   ,
    1_000_000 ,
    10_000_000,
];

use rayon::prelude::*;
pub trait Method where Self:Sized {
    // type Args;
    fn shot(&mut self, p: &Problem, credits: usize) -> ER;
    fn run (
        &mut self,
        p: &Problem,
        credits: usize,
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
            for credits in CREDITS {
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



//

pub mod grasp;
pub mod multistart;
pub mod experiment;

use experiment::Result as ER;
use experiment::Conclusion;
use crate::problem::Problem;
// pub mod simulated_annealing;

use rayon::prelude::*;
pub trait Method where Self:Sized {
    // type Args;
    fn shot(&mut self, p: &Problem) -> ER;
    fn run (
        &mut self,
        p: &Problem,
        num_shots: usize,
        problem_name: &str,
        method_name : &str,
    ) -> Conclusion {
        let mut shots = Vec::with_capacity(num_shots);
        for _ in 0..num_shots {
            let shot = self.shot(p);
            shots.push(shot);
        }

        Conclusion::new(
            shots,
            p,
            String::from(problem_name), 
            String::from(method_name),
        )
    }

    fn all_ms() -> Vec<(Self, String)>;
    fn run_all_ms_in_p(
        p: &Problem,
        problem_name: &str,
        num_shots_per_method: usize,
    ) -> Vec<Conclusion> {
        let ms = Self::all_ms();
        let mut ers = Vec::new();
        for (mut m, m_name) in ms.into_iter() {
            ers.push( m.run(
                p,
                num_shots_per_method,
                problem_name,
                &m_name,
            ));
        }

        ers
    }

    fn run_all_ms_in_all_ps(
        ps: Vec<(Problem, String)>,
        num_shots_per_method: usize,
    ) -> Vec<Conclusion> {
        assert_ne!(num_shots_per_method, 0);
        
        ps.into_par_iter()
            .map(|(p, p_name)| {
                println!("done with {:?}", &p_name);
                Self::run_all_ms_in_p(
                    &p,
                    &p_name,
                    num_shots_per_method,
                )
            })
            .collect::<Vec<Vec<Conclusion>>>()
            .into_iter()
            .flatten()
            .collect()
    }
}

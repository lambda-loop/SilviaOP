
// TODO: the add_cost calculations
// are not dealing very well with non-euclidian
// scenarios. To deal with it in a better way,
// it should consider that some costs could be
// actually negative, but thats not for now.

use crate::{
    Problem, Tour,
    problem::tour::Point,
};

// Thats was very AI Generated. Sowy.
pub fn naive_greedy(p: &Problem) -> Tour {
    let mut tour = Tour::new(p.len);

    let to_end: Vec<f64> = (0..p.len)
        .map(|i| p.costs[(i, 1)])
        .collect();

    let mut total_cost = to_end[0];
    let mut last = 0;

    loop {
        let last_to_end = to_end[last];
        let mut best: Option<(usize, f64, f64)> = None;

        for i in tour.avaiable() {
            let add_cost = p.costs[(last, i)]
                + to_end[i]
                - last_to_end;

            if total_cost + add_cost > p.tmax {
                continue;
            }

            if add_cost <= 0.0 {
                best = Some((i, f64::INFINITY, add_cost));
                break;
            }

            let perf = p.scores[i] as f64 / add_cost;

            match best {
                None => best = Some((i, perf, add_cost)),
                Some((_, best_perf, _)) if perf > best_perf => {
                    best = Some((i, perf, add_cost));
                }
                _ => {}
            }
        }

        let Some((i, _, add_cost)) = best else {
            break;
        };

        tour.push(i as Point);
        total_cost += add_cost;
        last = i;
    }

    tour
}

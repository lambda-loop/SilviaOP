use std::fs;

mod problem;
use problem::Problem;
use problem::heuristic as h;

mod rendering;
use rendering::{Map, window_conf};

#[macroquad::main(window_conf)]
async fn main() {
    // let mut out = None;

    let dir = fs::read_dir("data").expect("data dir");
    let mut maps = Vec::new();

    for entry in dir {
        if let Ok(dir_entry) = entry {
            let path = dir_entry.path();

            if path.extension().is_some_and(|ext| ext == "txt") {
                let input = fs::read_to_string(path).unwrap();

                let p = Problem::new(&input);

                let points = rendering::Point::problem(&input);

                let routes = problem::heuristic::apply_all(&p);

                for (method, route) in routes {
                    let status = p.eval_route(&route);
                    let title = format!("{:?} - {}", dir_entry.file_name(), method);


                let map = Map {
                    route,
                    points: points.clone(),
                    tmax: p.tmax,
                    used_cost: status.total_consume,
                    name: title,
                    score: status.total_score,
                };

                maps.push(map);
                    
                }

                // let status = p.eval_route(&route);
                // let method = "Greedy".to_string();
                // let title = format!("{:?} - {}", dir_entry.file_name(), method);

            } else {
                println!("{:?}", path);
            }
        }
    }

    // let map = out.unwrap();

    rendering::Renderer::new(maps)
        .run()
        .await;
}

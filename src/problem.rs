//

mod matrix;
mod pos;
use matrix::SquareMatrix;
use pos::Node as Pos2D;

#[derive(Debug)]
pub struct Problem {
    tmax: f32,
    costs: SquareMatrix<f32>,
    scores: Vec<u16>,
    len: usize, // num_points
}

impl Problem {
    pub fn new(raw_input: &str) -> Self {
        //
        let mut lines = raw_input.lines();
        let mut fst_line = lines.next().unwrap().trim().split_whitespace();

        let tmax: f32 = fst_line.next().unwrap().parse().unwrap();

        let mut points = Vec::new();
        for line in lines {
            let point = Pos2D::parse(line);
            points.push(point);
        }

        let len = points.len();
        let scores = points.iter().map(|p| p.score).collect();

        let mut costs = SquareMatrix::<f32>::new(len);

        for i in 0..len {
            for j in 0..len {
                costs[(i, j)] = points[i].euc_dist(&points[j]);
            }
        }

        Problem {
            tmax,
            costs,
            scores,
            len,
        }
    }
}

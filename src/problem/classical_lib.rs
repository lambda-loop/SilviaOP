
use super::*;

pub struct Node {
    x: f64,
    y: f64,
    pub score: u64,
}

impl Node {
    pub fn euc_dist(&self, other: &Self) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;

        dx.hypot(dy)
    }

    pub fn parse(line: &str) -> Self {
        let mut iter = line.split_ascii_whitespace();
        let x = iter.next().unwrap().parse::<f64>().unwrap();
        let y = iter.next().unwrap().parse::<f64>().unwrap();
        let score = iter.next().unwrap().parse::<u64>().unwrap();

        Self { x, y, score }
    }
}

pub fn parse(raw_input: &str) -> Problem {
    let mut lines = raw_input.lines();
    let mut fst_line = lines.next().unwrap().trim().split_whitespace();

    let tmax: f64 = fst_line.next().unwrap().parse().unwrap();

    let mut points = Vec::new();
    for line in lines {
        let point = Node::parse(line);
        points.push(point);
    }

    let len = points.len();
    let scores = points.iter().map(|p| p.score as u16).collect();

    let mut costs = Grid::<f64>::new(len, len);

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


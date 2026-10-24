//

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

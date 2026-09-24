//

pub struct Node {
    x: f32,
    y: f32,
    pub score: u32,
}

impl Node {
    pub fn euc_dist(&self, other: &Self) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;

        dx.hypot(dy)
    }

    pub fn parse(line: &str) -> Self {
        let mut iter = line.split_ascii_whitespace();
        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        let score = iter.next().unwrap().parse::<u32>().unwrap();

        Self { x, y, score }
    }
}

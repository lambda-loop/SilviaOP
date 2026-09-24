//

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub score: u16,
}

impl Point {
    pub fn parse(line: &str) -> Self {
        let mut iter = line.split_ascii_whitespace();

        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        let score = iter.next().unwrap().parse::<u16>().unwrap();

        Self { x, y, score }
    }

    pub fn parse_all(raw_input: &str) -> Vec<Point> {
        let mut lines = raw_input.lines();

        _ = lines.next();

        let mut points = Vec::new();

        for line in lines {
            let point = Point::parse(line);
            points.push(point);
        }

        points
    }
}

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

    pub fn parse_classical_coords(raw_input: &str) -> Vec<Point> {
        let mut lines = raw_input.lines();

        _ = lines.next();

        let mut points = Vec::new();

        for line in lines {
            let point = Point::parse(line);
            points.push(point);
        }

        points
    }

    pub fn parse_oplib_coords(raw_input: &str) -> Vec<Point> {
        let n = raw_input
            .lines()
            .find_map(|line| {
                let (key, value) = line.split_once(':')?;
                
                (key.trim() == "DIMENSION")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .expect("Missing DIMENSION");
        
        let section = |name: &str| {
            let mut lines = raw_input
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty());
            
            assert!(
                lines.by_ref().any(|line| line == name),
                "Missing {name}"
            );
            
            lines.take(n)
        };
        
        let mut coords = vec![None; n];
        let mut scores = vec![None; n];
        
        for line in section("NODE_COORD_SECTION") {
            let mut fields = line.split_ascii_whitespace();
            
            let id = fields.next().unwrap().parse::<usize>().unwrap();
            let x = fields.next().unwrap().parse::<f32>().unwrap();
            let y = fields.next().unwrap().parse::<f32>().unwrap();
            
            assert!(fields.next().is_none(), "Expected 2D coordinates");
            
            let index = id.checked_sub(1).expect("Invalid node ID");
            
            assert!(
                coords[index].replace((x, y)).is_none(),
                "Duplicate node coordinates"
            );
        }
        
        for line in section("NODE_SCORE_SECTION") {
            let mut fields = line.split_ascii_whitespace();
            
            let id = fields.next().unwrap().parse::<usize>().unwrap();
            let score = fields.next().unwrap().parse::<u16>().unwrap();
            
            assert!(fields.next().is_none(), "Expected one score per node");
            
            let index = id.checked_sub(1).expect("Invalid node ID");
            
            assert!(
                scores[index].replace(score).is_none(),
                "Duplicate node score"
            );
        }
        
        coords
            .into_iter()
            .zip(scores)
            .map(|(coords, score)| {
                let (x, y) = coords.expect("Missing node coordinates");
                let score = score.expect("Missing node score");
                
                Self { x, y, score }
            })
            .collect()
    }
}

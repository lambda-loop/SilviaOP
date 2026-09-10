//

#[derive(Debug, Clone)]
pub struct Result {
    pub problem_name: String,
    pub method: String,
    pub cost: f32,
    pub score: u16,
    pub route: Vec<u8>,
}

impl Result {
    pub fn to_csv(self) -> String {
        format!(
            "{},{},{},{:?}",
            self.method, self.cost, self.score, self.route,
        )
    }
}

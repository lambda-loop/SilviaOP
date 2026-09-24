//

#[derive(Debug, Clone)]
pub struct Result {
    pub problem_name: String,
    pub method: String,
    pub cost: f32,
    pub score: u32,
    pub route: Vec<u8>,
}

impl Result {
    pub fn to_csv(self) -> String {
        format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{:?}\"\n",
            self.problem_name, self.method, self.cost, self.score, self.route,
        )
    }

    pub fn header() -> String {
        String::from("problem_name,method,cost,score,route\n")
    }
}

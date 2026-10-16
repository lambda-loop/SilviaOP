//

// pub type Route = &[u8];

pub struct Tour<const N: usize> {
    buffer: [u8;N],
    len: u8,
}


#[derive(Debug, Clone, Copy)]
pub struct TourStatus {
    pub cost: f32,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct RouteStatus {
    pub total_score: u32,
    pub total_consume: f32,
}

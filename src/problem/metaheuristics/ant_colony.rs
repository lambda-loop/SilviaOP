
use grid::Grid;

pub struct ACO {
    alpha: f32,
    beta : f32,
    taff : Grid<f32>,
    rho  : f32,
    Q    : f32,
    t0   : f32,
}

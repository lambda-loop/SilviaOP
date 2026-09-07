//
pub mod point;
pub mod renderer;
// pub mod sa;
pub mod view;
// use sa::Map;
use point::Point;

use macroquad::prelude::*;

use renderer::Renderer;
pub trait Observer {
    fn init(raw_input: &str) -> Renderer;
    fn draw_aditional();
    fn run(&self, raw_input: &str) {
        let r = Self::init(raw_input);
    }
}

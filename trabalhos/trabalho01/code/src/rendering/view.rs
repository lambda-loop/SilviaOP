//

use super::point::*;
use macroquad::window::{screen_height, screen_width};

const MARGIN: f32 = 50.0;

#[derive(Debug, Clone)]
pub struct View {
    min_x: f32,
    min_y: f32,
    width: f32,
    height: f32,
}

impl View {
    pub fn new(ps: &[Point]) -> Self {
        // TODO: make them act in the same time?
        let min_x = ps.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
        let max_x = ps.iter().map(|p| p.x).fold(f32::NEG_INFINITY, f32::max);
        let min_y = ps.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
        let max_y = ps.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max);

        Self {
            min_x,
            min_y,
            width: (max_x - min_x).max(1.0),
            height: (max_y - min_y).max(1.0),
        }
    }

    pub fn to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        let available_w = screen_width() - MARGIN * 2.0;
        let available_h = screen_height() - MARGIN * 2.0;

        let scale = (available_w / self.width).min(available_h / self.height);

        let offset_x = MARGIN + (available_w - self.width * scale) / 2.0;

        let offset_y = MARGIN + (available_h - self.height * scale) / 2.0;

        let sx = (x - self.min_x) * scale + offset_x;
        let sy = (y - self.min_y) * scale + offset_y;

        (sx, sy)
    }
}

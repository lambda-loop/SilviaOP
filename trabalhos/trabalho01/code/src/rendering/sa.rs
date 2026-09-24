use super::*;
use crate::problem::Problem;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub route: Vec<u8>,
    pub points: Vec<Point>,
    pub tmax: f32,
    pub used_cost: f32,
    pub title: String,
    pub score: u16,
    pub temperature: f32,
    pub iteration: usize,
}

#[derive(Debug, Clone)]
struct Screen {
    view: View,
    map: Map,
}

pub trait Voyeur {
    fn init(raw_input: &str, r: Vec<u8>) -> Map;
    fn next(&mut self) -> Map;

    async fn run(&mut self, raw_input: &str, r: Vec<u8>) {
        let map = Self::init(raw_input, r);
        let mut screen = Screen::new(map);

        let mut last_step = get_time() - 1.0;

        loop {
            if is_key_pressed(KeyCode::Q) {
                break;
            }

            let now = get_time();

            if is_key_down(KeyCode::Right)
                // && now - last_step >= 0.00001
            {
                screen.map = self.next();
                last_step = now;
            }

            clear_background(BLACK);

            screen.draw();

            next_frame().await;
        }
    }
}
impl Screen {
    pub fn new(map: Map) -> Self {
        let view = View::new(&map.points);

        Self {
            view,
            map,
        }
    }

    pub fn draw(&self) {
        let r = Renderer {
            maps: vec![self.map.clone()],
            current: 0,
            view: self.view.clone(),
        };

        r.draw_points(&self.map);
        r.draw_route(&self.map);
        r.draw_route_points(&self.map);
        r.draw_info(&self.map);
    }
}

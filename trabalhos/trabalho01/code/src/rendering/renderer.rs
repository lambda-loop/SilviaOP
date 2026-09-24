//

use super::point::*;
use super::view::*;

#[derive(Debug, Clone)]
pub struct Map {
    pub route: Vec<u8>,
    pub points: Vec<Point>,
    pub tmax: f32,
    pub used_cost: f32,
    pub title: String,
    pub route_score: u32,
    // additional info
}

pub struct Renderer {
    pub map: Map,
    view: View,
}

use macroquad::prelude::*;
impl Renderer {
    pub fn new(map: Map) -> Self {
        let view = View::new(&map.points);

        Self { map, view }
    }

    pub fn draw(&self) {
        let map = &self.map;

        self.draw_points(map);
        self.draw_route(map);
        self.draw_route_points(map);
        self.draw_info(map);
    }

    fn draw_points(&self, map: &Map) {
        for p in &map.points {
            let (x, y) = self.view.to_screen(p.x, p.y);

            draw_circle_lines(x, y, 12.0, 2.0, WHITE);

            let score_str = format!("{}", p.score);
            let font_size = 14.0;
            let text_dims = measure_text(&score_str, None, font_size as u16, 1.0);

            let text_x = x - text_dims.width / 2.0;
            let text_y = y + text_dims.height / 3.0;

            draw_text(&score_str, text_x, text_y, font_size, WHITE);
        }
    }

    fn draw_route(&self, map: &Map) {
        let mut previous = 0u8;

        for next in map
            .route
            .iter()
            .copied()
            .chain(std::iter::once(1u8))
        {
            let pl = &map.points[previous as usize];
            let pr = &map.points[next as usize];

            let (xl, yl) = self.view.to_screen(pl.x, pl.y);
            let (xr, yr) = self.view.to_screen(pr.x, pr.y);

            draw_line(xl, yl, xr, yr, 3.0, BLUE);
            previous = next;
        }
    }

    fn draw_route_points(&self, map: &Map) {
        for &idx in &map.route {
            let p = &map.points[idx as usize];
            let (x, y) = self.view.to_screen(p.x, p.y);

            draw_circle(x, y, 10.0, BLUE);
            draw_circle_lines(x, y, 12.0, 2.0, WHITE);
        }

        self.draw_special_point(map, 0);
        self.draw_special_point(map, 1);
    }

    fn draw_special_point(&self, map: &Map, idx: u8) {
        let p = &map.points[idx as usize];
        let (x, y) = self.view.to_screen(p.x, p.y);

        draw_circle(x, y, 14.0, RED);
        draw_circle_lines(x, y, 16.0, 2.0, WHITE);
    }

    fn draw_info(&self, map: &Map) {
        let font_size = 24.0;

        let title_dims = measure_text(&map.title, None, font_size as u16, 1.0);
        let title_x = (screen_width() - title_dims.width) / 2.0;

        draw_text(&map.title, title_x, 30.0, font_size, WHITE);
        let score_text = format!("Score: {}", map.route_score);

        draw_text(&score_text, 20.0, screen_height() - 50.0, 20.0, WHITE);
        let cost_text = format!("{:.2} / {:.2}", map.used_cost, map.tmax);
        let cost_dims = measure_text(&cost_text, None, 20, 1.0);
        let cost_x = screen_width() - cost_dims.width - 20.0;
        let cost_color = if map.used_cost <= map.tmax {
            GREEN
        } else {
            RED
        };

        draw_text(&cost_text, cost_x, screen_height() - 50.0, 20.0, cost_color);
    }
}

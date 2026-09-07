//
pub mod point;
pub mod renderer;
pub mod view;
pub mod sa;
use sa::Map;
// AI GENERATED: (causes just a auxiliar and not acctally the ureal project)

use point::Point;

use macroquad::prelude::*;


#[derive(Debug, Clone)]
struct View {
    min_x: f32,
    min_y: f32,
    width: f32,
    height: f32,
}

impl View {
    fn new(points: &[Point]) -> Self {
        let min_x = points
            .iter()
            .map(|p| p.x)
            .fold(f32::INFINITY, f32::min);

        let max_x = points
            .iter()
            .map(|p| p.x)
            .fold(f32::NEG_INFINITY, f32::max);

        let min_y = points
            .iter()
            .map(|p| p.y)
            .fold(f32::INFINITY, f32::min);

        let max_y = points
            .iter()
            .map(|p| p.y)
            .fold(f32::NEG_INFINITY, f32::max);

        Self {
            min_x,
            min_y,
            width: (max_x - min_x).max(1.0),
            height: (max_y - min_y).max(1.0),
        }
    }

    fn to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        let margin = 50.0;

        let available_w = screen_width() - margin * 2.0;
        let available_h = screen_height() - margin * 2.0;

        let scale = (available_w / self.width)
            .min(available_h / self.height);

        let offset_x =
            margin + (available_w - self.width * scale) / 2.0;

        let offset_y =
            margin + (available_h - self.height * scale) / 2.0;

        let sx = (x - self.min_x) * scale + offset_x;
        let sy = (y - self.min_y) * scale + offset_y;

        (sx, sy)
    }
}

pub struct Renderer {
    maps: Vec<Map>,
    current: usize,
    view: View,
}

impl Renderer {
    pub fn new(maps: Vec<Map>) -> Self {
        assert!(!maps.is_empty());

        let view = View::new(&maps[0].points);

        Self {
            maps,
            current: 0,
            view,
        }
    }

    pub async fn run(mut self) {
        loop {
            if is_key_pressed(KeyCode::Q) {
                break;
            }

            if is_key_pressed(KeyCode::Right) {
                self.next();
            }

            if is_key_pressed(KeyCode::Left) {
                self.previous();
            }

            clear_background(BLACK);

            self.draw();

            next_frame().await;
        }
    }

    fn next(&mut self) {
        self.current = (self.current + 1) % self.maps.len();

        self.view = View::new(
            &self.maps[self.current].points
        );
    }

    fn previous(&mut self) {
        self.current =
            (self.current + self.maps.len() - 1) % self.maps.len();

        self.view = View::new(
            &self.maps[self.current].points
        );
    }

    fn draw(&self) {
        let map = &self.maps[self.current];

        self.draw_points(map);
        self.draw_route(map);
        self.draw_route_points(map);
        self.draw_info(map);
    }

    fn draw_points(&self, map: &Map) {
        for point in &map.points {
            let (x, y) =
                self.view.to_screen(point.x, point.y);

            draw_circle_lines(
                x,
                y,
                12.0,
                2.0,
                WHITE,
            );

            let score_str =
                format!("{}", point.score);

            let font_size = 14.0;

            let text_dims = measure_text(
                &score_str,
                None,
                font_size as u16,
                1.0,
            );

            let text_x =
                x - text_dims.width / 2.0;

            let text_y =
                y + text_dims.height / 3.0;

            draw_text(
                &score_str,
                text_x,
                text_y,
                font_size,
                WHITE,
            );
        }
    }

    fn draw_route(&self, map: &Map) {
        if map.route.is_empty() {
            return;
        }

        let mut full_route =
            Vec::with_capacity(map.route.len() + 2);

        full_route.push(0);
        full_route.extend_from_slice(&map.route);
        full_route.push(1);

        for pair in full_route.windows(2) {
            let p1 =
                &map.points[pair[0] as usize];

            let p2 =
                &map.points[pair[1] as usize];

            let (x1, y1) =
                self.view.to_screen(p1.x, p1.y);

            let (x2, y2) =
                self.view.to_screen(p2.x, p2.y);

            draw_line(
                x1,
                y1,
                x2,
                y2,
                3.0,
                BLUE,
            );
        }
    }

    fn draw_route_points(&self, map: &Map) {
        for &idx in &map.route {
            let point =
                &map.points[idx as usize];

            let (x, y) =
                self.view.to_screen(point.x, point.y);

            draw_circle(
                x,
                y,
                10.0,
                BLUE,
            );

            draw_circle_lines(
                x,
                y,
                12.0,
                2.0,
                WHITE,
            );
        }

        // Origem
        self.draw_special_point(map, 0);

        // Destino
        self.draw_special_point(map, 1);
    }

    fn draw_special_point(
        &self,
        map: &Map,
        idx: u8,
    ) {
        let point =
            &map.points[idx as usize];

        let (x, y) =
            self.view.to_screen(point.x, point.y);

        draw_circle(
            x,
            y,
            14.0,
            RED,
        );

        draw_circle_lines(
            x,
            y,
            16.0,
            2.0,
            WHITE,
        );
    }
fn draw_info(&self, map: &Map) {
    let font_size = 24.0;

    let title_dims = measure_text(
        &map.title,
        None,
        font_size as u16,
        1.0,
    );

    let title_x =
        (screen_width() - title_dims.width) / 2.0;

    draw_text(
        &map.title,
        title_x,
        30.0,
        font_size,
        WHITE,
    );

    let score_text =
        format!("Score: {}", map.score);

    draw_text(
        &score_text,
        20.0,
        screen_height() - 50.0,
        20.0,
        WHITE,
    );

    let cost_text =
        format!("{:.2} / {:.2}", map.used_cost, map.tmax);

    let cost_dims = measure_text(
        &cost_text,
        None,
        20,
        1.0,
    );

    let cost_x =
        screen_width() - cost_dims.width - 20.0;

    let cost_color =
        if map.used_cost <= map.tmax {
            GREEN
        } else {
            RED
        };

    draw_text(
        &cost_text,
        cost_x,
        screen_height() - 50.0,
        20.0,
        cost_color,
    );

    let iteration_text =
        format!("Iteration: {}", map.iteration);

    draw_text(
        &iteration_text,
        20.0,
        screen_height() - 20.0,
        20.0,
        WHITE,
    );

    let temperature_text =
        format!("Temperature: {:.4}", map.temperature);

    let temperature_dims = measure_text(
        &temperature_text,
        None,
        20,
        1.0,
    );

    let temperature_x =
        (screen_width() - temperature_dims.width) / 2.0;

    draw_text(
        &temperature_text,
        temperature_x,
        screen_height() - 20.0,
        20.0,
        WHITE,
    );
}

}


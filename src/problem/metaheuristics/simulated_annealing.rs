use crate::problem::local_search::*;
use crate::problem::route::RouteStatus;
use crate::problem::Problem;

use super::*;

use std::collections::HashSet;

pub struct State {
    problem: Problem,

    current: Vec<u8>,
    current_score: u16,
    current_consume: f32,

    best: Vec<u8>,
    best_score: u16,
    best_consume: f32,

    current_value: f32, // ??

    current_temp: f32,
    initial_temp: f32,
    cooling: f32,
    min_temp: f32,

    penalty: f32,
    penalty_power: f32,

    unvisited: HashSet<u8>,

    // points: Vec<Point>,
    iteration: usize,
}
impl State {
    pub fn new(problem: Problem, initial: Vec<u8>, unvisited: HashSet<u8>) -> Self {
        let status = problem.eval_route(&initial);

        let initial_temp = 100.0;

        let current_value = Self::value(
            status.total_score,
            status.total_consume,
            problem.tmax,
            initial_temp,
            initial_temp,
            1.0,
            2.0,
        );

        Self {
            problem,

            current: initial.clone(),
            best: initial,

            current_score: status.total_score,
            best_score: status.total_score,

            current_consume: status.total_consume,
            best_consume: status.total_consume,

            current_value,

            temp: initial_temp,
            initial_temp,
            cooling: 0.999, //0.995,
            min_temp: 0.01,

            penalty: 1.0,
            penalty_power: 2.0,

            unvisited,

            points,
            iteration: 0,
        }
    }
    fn value(
        score: u16,
        consume: f32,
        tmax: f32,
        temp: f32,
        initial_temp: f32,
        penalty: f32,
        penalty_power: f32,
    ) -> f32 {
        let violation = (consume - tmax).max(0.0);

        let temperature_factor = initial_temp / temp;

        let penalty = penalty * temperature_factor * violation.powf(penalty_power);

        score as f32 - penalty
    }

    fn candidate_value(&self, status: &RouteStatus) -> f32 {
        Self::value(
            status.total_score,
            status.total_consume,
            self.problem.tmax,
            self.temp,
            self.initial_temp,
            self.penalty,
            self.penalty_power,
        )
    }

    fn accept(&self, candidate_value: f32) -> bool {
        if candidate_value >= self.current_value {
            return true;
        }

        if self.temp <= self.min_temp {
            return false;
        }

        let delta = candidate_value - self.current_value;
        let probability = (delta / self.temp).exp();

        rand::random::<f32>() < probability
    }

    fn update_best(&mut self, candidate: &[u8], status: &RouteStatus) {
        if status.total_consume <= self.problem.tmax && status.total_score > self.best_score {
            self.best = candidate.to_vec();
            self.best_score = status.total_score;
            self.best_consume = status.total_consume;
        }
    }

    fn cool(&mut self) {
        self.temp *= self.cooling;

        if self.temp < self.min_temp {
            self.temp = self.min_temp;
        }
    }

    fn neighbor(&self) -> (Vec<u8>, HashSet<u8>) {
        match rand::random_range(0..6) {
            0 => (swap(&self.current), self.unvisited.clone()),

            1 => (two_opt(&self.current), self.unvisited.clone()),

            2 => (relocate(&self.current), self.unvisited.clone()),

            3 => insert(&self.current, &self.unvisited, self.problem.len as u8),

            4 => remove(&self.current, &self.unvisited),

            5 => replace(&self.current, &self.unvisited, self.problem.len as u8),

            _ => unreachable!(),
        }
    }

    pub fn step(&mut self) {
        self.iteration += 1;

        let (candidate, candidate_unvisited) = self.neighbor();

        let status = self.problem.eval_route(&candidate);

        self.update_best(&candidate, &status);

        let candidate_value = self.candidate_value(&status);

        if self.accept(candidate_value) {
            self.current = candidate;
            self.unvisited = candidate_unvisited;

            self.current_score = status.total_score;
            self.current_consume = status.total_consume;
            self.current_value = candidate_value;
        }

        self.cool();
    }
    pub fn run_sa(&mut self) -> Vec<u8> {
        while self.temp > self.min_temp {
            self.step();
        }

        self.best.clone()
    }

    pub fn current(&self) -> &[u8] {
        &self.current
    }

    pub fn best(&self) -> &[u8] {
        &self.best
    }

    pub fn current_score(&self) -> u16 {
        self.current_score
    }

    pub fn best_score(&self) -> u16 {
        self.best_score
    }

    pub fn current_consume(&self) -> f32 {
        self.current_consume
    }

    pub fn best_consume(&self) -> f32 {
        self.best_consume
    }

    pub fn temperature(&self) -> f32 {
        self.temp
    }
}

use crate::rendering::sa::*;
use crate::rendering::*;
use crate::rendering::*;

impl Voyeur for State {
    fn init(raw_input: &str, r: Vec<u8>) -> Map {
        let problem = Problem::new(raw_input);
        let points = Point::problem(raw_input);
        let status = problem.eval_route(&r);

        Map {
            route: r,
            points,
            tmax: problem.tmax,
            used_cost: status.total_consume,
            title: "Simulated Annealing".to_string(),
            score: status.total_score,
            temperature: 100.0,
            iteration: 0,
        }
    }

    fn next(&mut self) -> Map {
        self.step();

        Map {
            route: self.current.clone(),
            points: self.points.clone(),
            tmax: self.problem.tmax,
            used_cost: self.current_consume,
            title: "Simulated Annealing".to_string(),
            score: self.current_score,
            temperature: self.temp,
            iteration: self.iteration,
        }
    }
}

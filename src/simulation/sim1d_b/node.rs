use rand::distributions::Distribution;
use rand_distr::{Exp, Exp1};

use super::utils::*;

pub struct Node {
    t_heal: f64,
    t_infect_left: f64,
    t_infect_right: f64,
    t_infect_two_left: f64,
    t_infect_two_right: f64,
    pub state: State,
}


impl Node {
    pub fn new(exp_lr: &Exp<f64>, exp_two_lr: &Exp<f64>, t: f64) -> Self {
        let mut t_heal = -1.0;
        let mut t_infect_left = -1.0;
        let mut t_infect_right = -1.0;
        let mut t_infect_two_left = -1.0;
        let mut t_infect_two_right = -1.0;
        while t_heal < t {
            let random_number: f64 = Exp1.sample(&mut rand::thread_rng());
            t_heal += random_number
        }
        while t_infect_left < t {
            t_infect_left += exp_lr.sample(&mut rand::thread_rng());
        }

        while t_infect_right < t {
            t_infect_right += exp_lr.sample(&mut rand::thread_rng());
        }

        while t_infect_two_left < t {
            t_infect_two_left += exp_two_lr.sample(&mut rand::thread_rng());
        }

        while t_infect_two_right < t {
            t_infect_two_right += exp_two_lr.sample(&mut rand::thread_rng());
        }

        Node {
            t_heal,
            t_infect_left,
            t_infect_right,
            t_infect_two_left,
            t_infect_two_right,
            state: State::Infected,
        }
    }

    pub fn get_min(&self) -> f64 {
        let values = [
            self.t_heal,
            self.t_infect_left,
            self.t_infect_right,
            self.t_infect_two_left,
            self.t_infect_two_right,
        ];

        *values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    pub fn get_min_state(&self) -> InfectProgress {
        if self.t_heal < self.t_infect_left && self.t_heal < self.t_infect_right && self.t_heal < self.t_infect_two_left && self.t_heal < self.t_infect_two_right {
            InfectProgress::Heal
        } else if self.t_infect_left < self.t_infect_right && self.t_infect_left < self.t_infect_two_left && self.t_infect_left < self.t_infect_two_right {
            InfectProgress::Left
        } else if self.t_infect_right < self.t_infect_two_left && self.t_infect_right < self.t_infect_two_right {
            InfectProgress::Right
        } else if self.t_infect_two_left < self.t_infect_two_right {
            InfectProgress::TwoLeft
        } else {
            InfectProgress::TwoRight
        }
    }


    fn heal(&mut self, exp_lr: &Exp<f64>, exp_two_lr: &Exp<f64>) {
        while self.t_infect_left < self.t_heal {
            self.t_infect_left += exp_lr.sample(&mut rand::thread_rng());
        }
        while self.t_infect_right < self.t_heal {
            self.t_infect_right += exp_lr.sample(&mut rand::thread_rng());
        }
        while self.t_infect_two_left < self.t_heal {
            self.t_infect_two_left += exp_two_lr.sample(&mut rand::thread_rng());
        }
        while self.t_infect_two_right < self.t_heal {
            self.t_infect_two_right += exp_two_lr.sample(&mut rand::thread_rng());
        }

        let random_number: f64 = Exp1.sample(&mut rand::thread_rng());
        self.t_heal += random_number;
        self.state = State::Healthy;
    }

    pub fn infect(&mut self, exp_lr: &Exp<f64>, exp_two_lr: &Exp<f64>, t: f64) {
        self.state = State::Infected;
        while self.t_heal < t {
            let random_number: f64 = Exp1.sample(&mut rand::thread_rng());
            self.t_heal += random_number;
        }
        while self.t_infect_left < t {
            self.t_infect_left += exp_lr.sample(&mut rand::thread_rng());
        }
        while self.t_infect_right < t {
            self.t_infect_right += exp_lr.sample(&mut rand::thread_rng());
        }
        while self.t_infect_two_left < t {
            self.t_infect_two_left += exp_two_lr.sample(&mut rand::thread_rng());
        }
        while self.t_infect_two_right < t {
            self.t_infect_two_right += exp_two_lr.sample(&mut rand::thread_rng());
        }
    }

    pub fn step(&mut self, exp_lr: &Exp<f64>, exp_ud: &Exp<f64>) {
        match self.get_min_state() {
            InfectProgress::Heal => { self.heal(exp_lr, exp_ud) },
            InfectProgress::Left => { self.t_infect_left += exp_lr.sample(&mut rand::thread_rng())},
            InfectProgress::Right => { self.t_infect_right += exp_lr.sample(&mut rand::thread_rng())},
            InfectProgress::TwoLeft => { self.t_infect_two_left += exp_ud.sample(&mut rand::thread_rng())},
            InfectProgress::TwoRight => { self.t_infect_two_right += exp_ud.sample(&mut rand::thread_rng())},
        }
    }
}
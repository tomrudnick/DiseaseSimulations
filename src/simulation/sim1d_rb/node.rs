use rand::distributions::Distribution;
use rand_distr::{Exp, Exp1};

use super::utils::*;


#[derive(Copy, Clone)]
pub struct Node {
    pub t_heal: f64,
    pub t_infect_left: f64,
    pub t_infect_right: f64,
    pub state: State,
}


impl Node {
    pub fn new(exp: &Exp<f64>, t: f64) -> Self {
        let mut t_heal = -1.0;
        let mut t_infect_left = -1.0;
        let mut t_infect_right = -1.0;
        while t_heal < t {
            let random_number: f64 = Exp1.sample(&mut rand::thread_rng());
            t_heal += random_number
        }
        while t_infect_left < t {
            t_infect_left += exp.sample(&mut rand::thread_rng());
        }
        while t_infect_right < t {
            t_infect_right += exp.sample(&mut rand::thread_rng());
        }

        Node {
            t_heal,
            t_infect_left,
            t_infect_right,
            state: State::Infected,
        }
    }

    pub fn get_min(&self) -> f64 {
        if self.t_heal < self.t_infect_left && self.t_heal < self.t_infect_right {
            self.t_heal
        } else if self.t_infect_left < self.t_infect_right {
            self.t_infect_left
        } else {
            self.t_infect_right
        }
    }

    pub fn get_min_state(&self) -> InfectProgress {
        if self.t_heal < self.t_infect_left && self.t_heal < self.t_infect_right {
            InfectProgress::Heal
        } else if self.t_infect_left < self.t_infect_right {
            InfectProgress::Left
        } else {
            InfectProgress::Right
        }
    }

    fn heal(&mut self, exp: &Exp<f64>) {
        while self.t_infect_left < self.t_heal {
            self.t_infect_left += exp.sample(&mut rand::thread_rng());
        }
        while self.t_infect_right < self.t_heal {
            self.t_infect_right += exp.sample(&mut rand::thread_rng());
        }

        let random_number: f64 = Exp1.sample(&mut rand::thread_rng());
        self.t_heal += random_number;
        self.state = State::Healthy;
    }

    pub fn infect(&mut self, exp: &Exp<f64>, t: f64) {
        self.state = State::Infected;
        while self.t_heal < t {
            let random_number: f64 = Exp1.sample(&mut rand::thread_rng());
            self.t_heal += random_number;
        }
        while self.t_infect_left < t {
            self.t_infect_left += exp.sample(&mut rand::thread_rng());
        }
        while self.t_infect_right < t {
            self.t_infect_right += exp.sample(&mut rand::thread_rng());
        }
    }

    pub fn step(&mut self, exp: &Exp<f64>) {
        match self.get_min_state() {
            InfectProgress::Left => {
                self.t_infect_left += exp.sample(&mut rand::thread_rng());
            },
            InfectProgress::Right => {
                self.t_infect_right += exp.sample(&mut rand::thread_rng());
            },
            InfectProgress::Heal => {
                self.heal(exp);
            }
        }
    }
}


impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.state.partial_cmp(&other.state)
            .and_then(|ord| {
                match ord {
                    std::cmp::Ordering::Equal => {
                        self.get_min().partial_cmp(&other.get_min())
                    },
                    _ => Some(ord)
                }
            })
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.get_min() == other.get_min() && self.state == other.state
    }
}
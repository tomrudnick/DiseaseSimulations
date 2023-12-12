
use std::collections::HashMap;
use rand_distr::Exp;


mod node;
mod utils;


use node::Node;
use utils::State;
use utils::InfectProgress;

use super::sim::*;

pub struct Simulation {
    nodes: HashMap<i32, Node>,
    t: f64,
    exp_lr: Exp<f64>,
    exp_two_lr: Exp<f64>
}

impl Sim for Simulation {

    fn run(&mut self, t_max: f64) -> bool {
        while self.t < t_max {
            let result = self.step();
            if !result {
                return true
            }
        }
        false
    }

    fn get_number_of_infected_nodes(&self) -> usize {
        self.nodes.iter().filter(|(_, v)| v.state == State::Infected).count()
    }


}



impl Simulation {

    pub(crate) fn new(lambda: f64, alpha: f64) -> Self {
        let mut nodes = HashMap::new();
        let exp_lr = Exp::new(lambda * alpha).unwrap();
        let exp_two_lr = Exp::new(lambda * (1.0 - alpha)).unwrap();
        let start_node = Node::new(&exp_lr, &exp_two_lr, 0.0);
        nodes.insert(0, start_node);
        Simulation {
            nodes,
            t: 0.0,
            exp_lr,
            exp_two_lr,
        }
    }
    fn step(&mut self) -> bool{
        let (key, min_t) = match self.nodes.iter()
            .filter(|&(_,v)| v.state == State::Infected)
            .map(|(k,v)| (k, v.get_min()))
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(&k, v)| (k, v))
        {
            Some(result) => result,
            None => {
                return false;
            }
        };
        //println!("Number of infected nodes: {} t: {}", self.get_number_of_infected_nodes(), self.t);
        //self.t += min_t;
        self.t = min_t;
        let node = self.nodes.get(&key);
        match node {
            Some(node) => {
                match node.get_min_state() {
                    InfectProgress::Left => { self.infect(key - 1) },
                    InfectProgress::Right => { self.infect(key + 1) },
                    InfectProgress::TwoLeft => { self.infect(key - 2) },
                    InfectProgress::TwoRight => { self.infect(key + 2) },
                    _ => {}
                }
                let node = self.nodes.get_mut(&key).unwrap();
                node.step(&self.exp_lr, &self.exp_two_lr);
            },
            None => {
                panic!("Node not found");
            }
        }

        true
    }

    fn infect(&mut self, key: i32) {
        let node = self.nodes.get_mut(&key);
        match node {
            Some(node) => {
                node.infect(&self.exp_lr, &self.exp_two_lr, self.t);
            },
            None => {
                let new_node = Node::new(&self.exp_lr, &self.exp_two_lr, self.t);
                self.nodes.insert(key, new_node);
            }
        }
    }
}


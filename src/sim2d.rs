
use std::collections::HashMap;
use rand_distr::Exp;


mod node;
mod utils;


use node::Node;
use utils::State;
use utils::InfectProgress;


pub struct Simulation {
    nodes: HashMap<(i32, i32), Node>,
    t: f64,
    exp_lr: Exp<f64>,
    exp_ud: Exp<f64>
}

impl Simulation {
    pub fn new(lambda: f64, alpha: f64) -> Self {
        let mut nodes = HashMap::new();
        let exp_lr = Exp::new(lambda * alpha).unwrap();
        let exp_ud = Exp::new(lambda * (1.0 - alpha)).unwrap();
        let start_node = Node::new(&exp_lr, &exp_ud, 0.0);
        nodes.insert((0, 0), start_node);
        Simulation {
            nodes,
            t: 0.0,
            exp_lr,
            exp_ud,
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
        let (x, y) = key;
        match node {
            Some(node) => {
                match node.get_min_state() {
                    InfectProgress::Left => { self.infect(x - 1, y) },
                    InfectProgress::Right => { self.infect(x + 1, y) },
                    InfectProgress::Up => { self.infect(x, y + 1) },
                    InfectProgress::Down => { self.infect(x, y - 1) },

                    _ => {}
                }
                let node = self.nodes.get_mut(&key).unwrap();
                node.step(&self.exp_lr, &self.exp_ud);
            },
            None => {
                panic!("Node not found");
            }
        }

        true
    }

    fn infect(&mut self, x: i32, y: i32) {
        let node = self.nodes.get_mut(&(x, y));
        match node {
            Some(node) => {
                node.infect(&self.exp_lr, &self.exp_ud, self.t);
            },
            None => {
                let node = Node::new(&self.exp_lr, &self.exp_ud, self.t);
                self.nodes.insert((x, y), node);
            }
        }
    }

    pub fn run(&mut self, t_max: f64) -> bool {
        while self.t < t_max {
            let result = self.step();
            if !result {
                return true
            }
        }
        false
    }

    pub fn get_number_of_infected_nodes(&self) -> usize {
        self.nodes.iter().filter(|(_, v)| v.state == State::Infected).count()
    }
}


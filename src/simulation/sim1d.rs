use std::collections::HashMap;
use rand_distr::Exp;




mod node;
mod utils;


use node::Node;
use utils::State;
use utils::InfectProgress;


pub struct Simulation {
    nodes: HashMap<i32, Node>,
    t: f64,
    exp: Exp<f64>
}

impl Simulation {
    pub fn new(lambda: f64) -> Self {
        let mut nodes = HashMap::new();
        let exp = Exp::new(lambda).unwrap();
        let start_node = Node::new(&exp, 0.0);
        nodes.insert(0, start_node);
        Simulation {
            nodes,
            t: 0.0,
            exp,
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

        //self.t += min_t;
        self.t = min_t;
        let node = self.nodes.get(&key);
        match node {
            Some(node) => {
                match node.get_min_state() {
                    InfectProgress::Left => { self.infect(key - 1) },
                    InfectProgress::Right => { self.infect(key + 1) },
                    _ => {}
                }
                let node = self.nodes.get_mut(&key).unwrap();
                node.step(&self.exp);
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
                node.infect(&self.exp, self.t);
            },
            None => {
                let new_node = Node::new(&self.exp, self.t);
                self.nodes.insert(key, new_node);
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


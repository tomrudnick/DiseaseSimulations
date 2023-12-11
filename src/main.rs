use std::collections::HashMap;
use std::fmt::Display;
use rand::Rng;
use rand_distr::{Exp, Distribution, Exp1};
use threadpool::ThreadPool;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;



#[derive(Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}


#[derive(PartialEq, Eq)]
enum InfectProgress {
    Left,
    Right,
    Heal,
}

#[derive(PartialEq, Eq)]
enum State {
    Infected,
    Healthy,
}

struct Node {
    t_heal: f64,
    t_infect_left: f64,
    t_infect_right: f64,
    state: State,
}

impl Node {
    fn new(exp: &Exp<f64>, t: f64) -> Self {
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

    fn get_min(&self) -> f64 {
        if self.t_heal < self.t_infect_left && self.t_heal < self.t_infect_right {
            self.t_heal
        } else if self.t_infect_left < self.t_infect_right {
            self.t_infect_left
        } else {
            self.t_infect_right
        }
    }

    fn get_min_state(&self) -> InfectProgress {
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

    fn infect(&mut self, exp: &Exp<f64>, t: f64) {
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

    fn step(&mut self, exp: &Exp<f64>) {
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

struct Simulation {
    nodes: HashMap<i32, Node>,
    t: f64,
    exp: Exp<f64>
}

impl Simulation {
    fn new(lambda: f64) -> Self {
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

        let count_infected = self.nodes.iter()
            .filter(|&(_,v)| v.state == State::Infected)
            .count();

        //println!("current_t {} min_t: {}, count: {}", self.t, min_t, count_infected);

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

    fn run(&mut self, t_max: f64) -> bool {
        while self.t < t_max {
            let result = self.step();
            if !result {
                return true
            }
        }
        false
    }
}


//step size is 0.01
fn run_simulation(simulations: i32, t_max: f64, lambda_range: (f64, f64)) {
    let mut simulation_results = Vec::new();
    let n_workers = 12;
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(n_workers);

    let lower_bound = (lambda_range.0 * 100.0) as i32;
    let upper_bound = (lambda_range.1 * 100.0) as i32;

    let simulations = simulations;
    for lambda in lower_bound..=upper_bound {
        let tx = tx.clone();

        pool.execute(move || {
            let lambda = lambda as f64 / 100.0;
            let mut success = 0;
            println!("Lambda: {}", lambda);
            for i in 0..simulations {
                let mut sim = Simulation::new(lambda);
                let result = sim.run(t_max);
                if result {
                    success += 1;
                }
            }
            tx.send((lambda, success)).unwrap();
        });
    }
    pool.join();
    drop(tx);
    for (lambda, success) in rx {
        simulation_results.push((lambda, success));
    }


    simulation_results.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    for (lambda, success) in simulation_results
    {
        println!("Lambda: {}, Success: {}", lambda, success as f64 / simulations as f64);
    }
}


fn main() {
    //let mut sim = Simulation::new(2.0);
    //sim.run(1000000f64);
    run_simulation(1000, 10000f64, (1.3, 1.7));
}

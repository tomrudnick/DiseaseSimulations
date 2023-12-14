use std::collections::HashMap;
use rand_distr::Exp;





mod node;
mod utils;

use node::Node;
use utils::InfectProgress;
use utils::State;

use super::rbtree::RBTree;
use super::rbtree::NodePtr;

#[derive(Copy, Clone)]
struct Value {
    v: i32,
    left: NodePtr<Node, Value>,
    right: NodePtr<Node, Value>,
}

pub struct Simulation {
    nodes: RBTree<Node, Value>,
    t: f64,
    exp: Exp<f64>
}

impl Simulation {
    pub fn new(lambda: f64) -> Self {
        let mut nodes = RBTree::new();
        let exp = Exp::new(lambda).unwrap();
        let start_node = Node::new(&exp, 0.0);
        let _ = nodes.insert(start_node, Value { v: 0, left: NodePtr::null(), right: NodePtr::null() });
        Simulation {
            nodes,
            t: 0.0,
            exp,
        }
    }

    fn step(&mut self) -> bool {
        let (option, ptr) = self.nodes.get_first_ptr();
        let (node, value) = match option {
            Some((n,v)) => if n.state == State::Infected { (n,v) } else { return false; },
            None => { return false; }
        };

        let min_t = node.get_min();
        self.t = min_t;

        let mut node = *node;
        let mut value = *value;



        match node.get_min_state() {
            InfectProgress::Left => { value.left = self.infect_left(&value, ptr) },
            InfectProgress::Right => { value.right = self.infect_right(&value, ptr) },
            _ => {}
        }

        self.node_step(&mut node, &value, ptr);

        //println!("Infected Nodes: {} t {}", self.get_number_of_infected_nodes(), self.t);
        true

    }

    fn node_step(&mut self, node: &mut Node, value: &Value, ptr: NodePtr<Node, Value>) {
        unsafe {
            node.step(&self.exp);
            self.nodes.delete(ptr);
            let ptr = self.nodes.insert(*node, *value);
            if !value.left.is_null() {
                let left = &mut (*value.left.0).value;
                left.right = ptr;
            }

            if !value.right.is_null() {
                let right = &mut (*value.right.0).value;
                right.left = ptr;
            }
        }
    }


    fn infect_left(&mut self, value: &Value, node_ptr: NodePtr<Node, Value>) -> NodePtr<Node, Value> {
        if value.left.is_null() {
            let new_node = Node::new(&self.exp, self.t);
            let new_value = Value { v: value.v - 1, left: NodePtr::null(), right: node_ptr };
            let new_ptr = self.nodes.insert(new_node, new_value);
            new_ptr
        } else {
            unsafe {
                let left = &(*value.left.0).key;
                let left_value = &(*value.left.0).value;
                let mut c_left = Node { ..*left };
                let c_left_value = Value { ..*left_value };

                c_left.infect(&self.exp, self.t);
                self.nodes.delete(value.left);
                let new_left_ptr = self.nodes.insert(c_left, c_left_value);
                new_left_ptr
            }
        }

    }

    fn infect_right(&mut self, value: &Value, node_ptr: NodePtr<Node, Value>) -> NodePtr<Node, Value>{
        if value.right.is_null() {
            let new_node = Node::new(&self.exp, self.t);
            let new_value = Value { v: value.v + 1, left: node_ptr, right: NodePtr::null() };
            let new_ptr = self.nodes.insert(new_node, new_value);
            new_ptr
        } else {
            unsafe {
                let right = &(*value.right.0).key;
                let right_value = &(*value.right.0).value;
                let mut c_right = Node { ..*right };
                let c_right_value = Value { ..*right_value };

                c_right.infect(&self.exp, self.t);
                self.nodes.delete(value.right);
                let new_right_ptr = self.nodes.insert(c_right, c_right_value);
                new_right_ptr
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
        self.nodes.iter().filter(|(k, v)| k.state == State::Infected).count()
    }

}


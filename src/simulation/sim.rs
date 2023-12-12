use super::sim2d;
use super::sim1d;
use super::sim1d_b;

pub trait Sim {
    fn run(&mut self, t_max: f64) -> bool;
    fn get_number_of_infected_nodes(&self) -> usize;

}

#[derive(Clone, Copy)]
pub enum SimAlphaType {
    TwoD,
    OneDB,
}

#[inline]
pub fn create_sim(sim_type: SimAlphaType, lambda: f64, alpha: f64) -> Box<dyn Sim> {
    match sim_type {
        SimAlphaType::TwoD => Box::new(sim2d::Simulation::new(lambda, alpha)),
        SimAlphaType::OneDB => Box::new(sim1d_b::Simulation::new(lambda, alpha)),
    }
}
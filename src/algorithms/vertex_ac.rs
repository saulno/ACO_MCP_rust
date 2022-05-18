use std::collections::HashSet;

use crate::models::parameters::{Parameters};

use super::aco::ACO;


pub struct VertexAC {
    pheromones: Vec<f32>
}

impl VertexAC {
    pub fn new(parameters: &Parameters) -> VertexAC {
        let mut v = VertexAC { pheromones: vec![] };
        v.set_initial_pheromone_trails(parameters);

        v
    }
}

impl ACO for VertexAC {
    fn set_initial_pheromone_trails(&mut self, p: &Parameters) {
        self.pheromones = vec![p.tau_max; p.graph.n_vertex];
    }

    fn tau_factor_of_vertex(&self, vertex: &usize, _current_clique: &HashSet<usize>) -> f32 {
        self.pheromones[*vertex]
    }

    fn increment_pheromone(&mut self, p: &Parameters, pheromone_delta: &f32, _current_clique: &HashSet<usize>) {
        for i in 0..self.pheromones.len() {
            self.pheromones[i] += pheromone_delta;
            if self.pheromones[i] > p.tau_max {
                self.pheromones[i] = p.tau_max;
            }
        }
    }

    fn decrement_pheromone(&mut self, p: &Parameters) {
        for i in 0..self.pheromones.len() {
            self.pheromones[i] *= p.rho;
            if self.pheromones[i] < p.tau_min {
                self.pheromones[i] = p.tau_min;
            }
        }
    }
}
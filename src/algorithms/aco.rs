use std::{collections::HashSet};

use rand::Rng;

use crate::models::parameters::{Parameters};

pub trait ACO {
    fn set_initial_pheromone_trails(&mut self, p: &Parameters);
    
    fn tau_factor_of_vertex(&self, vertex: &usize, current_clique: &HashSet<usize>) -> f32;

    fn increment_pheromone(&mut self, p: &Parameters, pheromone_delta: &f32, current_clique: &HashSet<usize>);

    fn decrement_pheromone(&mut self, p: &Parameters);

    fn update_pheromone_trail(&mut self, p: &Parameters, global_best_clique: &HashSet<usize>, k_best_clique: &HashSet<usize>) {
        self.decrement_pheromone(p);
        let pheromone_delta = 1.0 / (1.0 + global_best_clique.len() as f32 - k_best_clique.len() as f32);
        self.increment_pheromone(p, &pheromone_delta, k_best_clique);
    }
    
    fn vertex_probability(&self, p: &Parameters, vertex: &usize, current_clique: &HashSet<usize>) -> f32 {
        let tau = self.tau_factor_of_vertex(vertex, current_clique).powf(p.alpha);
        let mut sum_others_tau: f32 = 0.0;
        for candidate in current_clique.into_iter() {
            sum_others_tau += self.tau_factor_of_vertex(candidate, current_clique);
        }
        tau / sum_others_tau
    }

    fn choose_vertex_using_pheromones_probabilities(&self, p: &Parameters, candidates: &HashSet<usize>, current_clique: &HashSet<usize>) -> usize {
        let mut probabilities: Vec<(usize, f32)> = vec![(0, 0.0); candidates.len()]; 
        let mut sum = 0.0;
        for (i, candidate) in candidates.into_iter().enumerate() {
            let p = self.vertex_probability(p, candidate, current_clique);
            sum += p;
            probabilities[i] = (*candidate, sum);
        }
        
        let random: f32 = rand::thread_rng().gen_range(0.0..1.0);
        for i in 0..probabilities.len() {
            if random <= probabilities[i].1 {
                return probabilities[i].0
            }

        }

        probabilities.last().unwrap().0
    }

    fn aco_procedure(&mut self, p: &Parameters) -> HashSet<usize> {
        let mut global_best: HashSet<usize> = HashSet::new();

        for _gen in 0..p.cycles {
            let mut gen_best: HashSet<usize> = HashSet::new();

            for _k in 0..p.ants {
                let initial_vertex = p.graph.select_random_vertex();
                let mut k_clique: HashSet<usize> = HashSet::from_iter(vec![initial_vertex]);
                let mut candidates = p.graph.get_neighbor_candidates(initial_vertex);

                while candidates.len() > 0 {
                    let new_v = self.choose_vertex_using_pheromones_probabilities(p, &candidates, &k_clique);
                    let new_v_candidates = p.graph.get_neighbor_candidates(new_v);
                    k_clique.insert(new_v);
                    candidates = candidates.intersection(&new_v_candidates).map(|x| *x).collect();

                }

                gen_best = Self::choose_best_clique(&gen_best, &k_clique);
            }
            
            global_best = Self::choose_best_clique(&global_best, &gen_best);
            self.update_pheromone_trail(p, &global_best, &gen_best);
        }

        global_best
    }

    fn choose_best_clique(clique_1: &HashSet<usize>, clique_2: &HashSet<usize>) -> HashSet<usize> {
        if clique_1.len() > clique_2.len() {
            return clique_1.clone();
        } else if clique_1.len() == clique_2.len() {
            if rand::thread_rng().gen_ratio(1, 2) {
                return clique_1.clone();
            } else {
                return clique_2.clone();
            }
        }

        return clique_2.clone();
    }
    
}
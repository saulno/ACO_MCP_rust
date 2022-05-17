pub mod models;

use std::{error::Error, fs};
use models::config::Config;
use models::graph::Graph;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config::Help() => {
            println!("This is an implementation for the Ant Colony optimization algorithm described in http://dx.doi.org/10.1007/s10732-006-4295-8.");
            println!("It's designed to find the largest clique on a given graph.");
            println!("There are three options: help, vertex-ac, edge-ac");
            println!("      help                             It displays the  help.");
            println!("      vertex-ac [FILENAME] [...]       It runs the Vertex-AC variant of the algorithm (pheromone trails on vertices)");
            println!("      edge-ac [FILENAME] [...]         It runs the Edge-AC variant of the algorithm (pheromone trails on edges");
            println!("To run the algorithms, a series of parameters must be given in the following order:");
            println!("      cycles      The number of generations to run the algorithm");
            println!("      ants        The number of ants");
            println!("      alpha       The exponent to determine the relevance of the pheromone trails");
            println!("      rho         The evaporation rate for the pheromone trails");
            println!("      tau_max     The maximum ammount of pheromone on a given component");
            println!("      tau_min     The minimum ammount of pheromone on a given component");
            
        }
        Config::VertexAC(filename, cycles, ants, alpha, rho, tau_max, tau_min) => {
            println!("Vertex-AC: {} {} {} {} {} {} {}", filename, cycles, ants, alpha, rho, tau_max, tau_min);

            let contents = fs::read_to_string(filename)?;
            let graph = Graph::new(contents);

            println!("{:?}", graph);
        },
        Config::EdgeAC(filename, cycles, ants, alpha, rho, tau_max, tau_min) => {
            println!("Edge-AC: {} {} {} {} {} {} {}", filename, cycles, ants, alpha, rho, tau_max, tau_min);

            let contents = fs::read_to_string(filename)?;
            let graph = Graph::new(contents);

            println!("{:?}", graph);
        },
        
    }

    Ok(())
}
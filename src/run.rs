use crate::simulation::{Simulation,Parameters};
use crate::fitness::FitnessEvaluator;

struct DistanceEvaluator {}

impl FitnessEvaluator for DistanceEvaluator {
    fn evaluate(&self, code: &Vec<u8>) -> f64 {
        return code.iter().map(|&b| b as f64).sum();
    }
}

pub fn run() {
    let sim_params: Parameters<DistanceEvaluator> = Parameters{
        population_size: 100,
        genetic_code_length: 10,
        keep_threshold: 0.1,  // Keep top 10%.
        mutation_chance_percent: 0.01,
        fitness_evaluator: DistanceEvaluator{},
    };

    let mut sim = Simulation::new(sim_params);
    sim.run();

    println!("Bing Boom!");
}

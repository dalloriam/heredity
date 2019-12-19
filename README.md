# Gene

## Example binary
Here is an example binary that uses this crate.

```rust
use std::time::Instant;

extern crate gene;

struct DistanceEvaluator {}

impl gene::FitnessEvaluator for DistanceEvaluator {
    fn evaluate(&self, code: &Vec<u8>) -> f64 {
        return code.iter().map(|&b| b as f64).sum();
    }
}

fn main() {
    let sim_params: gene::SimulationParameters<DistanceEvaluator> = gene::SimulationParameters{
        population_size: 100,
        genetic_code_length: 10,
        keep_threshold: 0.5,  // Keep top 10%.
        mutation_chance_percent: 0.01,
        fitness_evaluator: DistanceEvaluator{},
    };

    let mut sim = gene::Simulation::new(sim_params);

    let start = Instant::now();
    sim.run();
    let duration = start.elapsed();

    println!("Time elapsed: {:?}", duration);
}
```

# Heredity

Heredity is a minimal general genetic algorithm library.

As long as your problem is representable as a `Vec<u8>` and you can assign a `f64` score to any given solution, Heredity can find a solution!

## Quickstart
```rust
use heredity::{FitnessEvaluator, Parameters, ThreadSim};

struct SumEvaluator {}

impl FitnessEvaluator for SumEvaluator {
    fn evaluate(&self, code: &Vec<u8>) -> f64 {
        return code.iter().map(|&b| b as f64).sum();
    }
}

pub fn main() {
    const MAX_IT_BEFORE_CONVERGE: usize = 10; // Each result is 1000 results, so this is really 10k iterations.

    let params = Parameters::new(SumEvaluator {});
    let expected_max_score = (params.genetic_code_length * (std::u8::MAX as usize)) as f64; // in our demo, the score of the genetic code is the sum of all bytes, so we can calculate the maximal score.
    let sim = ThreadSim::start(params);

    let result_stream = sim.results();

    let mut it_count = 0;
    let mut last_score = -1.0;

    loop {
        match result_stream.recv() {
            Ok(result) => {
                assert!(result.score >= last_score);
                last_score = result.score;

                // Success condition: within 2% of the maximum.
                if (result.score - expected_max_score).abs() <= (expected_max_score * 0.02) {
                    break;
                }
            }
            Err(_e) => {
                break;
            }
        }
        it_count += 1;

        if it_count > MAX_IT_BEFORE_CONVERGE {
            panic!("Took too long.");
        }
    }
    sim.stop().unwrap();
}

```

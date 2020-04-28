use gene::{FitnessEvaluator, Parameters, ThreadSim};

struct SumEvaluator {}

impl FitnessEvaluator for SumEvaluator {
    fn evaluate(&self, code: &Vec<u8>) -> f64 {
        return code.iter().map(|&b| b as f64).sum();
    }
}

#[test]
pub fn test_full_loop() {
    const MAX_IT_BEFORE_CONVERGE: usize = 10; // Each result is 1000 results, so this is really 10k its.

    let params = Parameters::new(SumEvaluator {});
    let expected_max_score = (params.genetic_code_length * (std::u8::MAX as usize)) as f64;
    let mut sim = ThreadSim::new();
    let result_rx = sim.start(params).unwrap();

    let mut it_count = 0;
    let mut last_score = -1.0;
    loop {
        match result_rx.recv() {
            Ok(result) => {
                assert!(result.score >= last_score);
                last_score = result.score;

                // Success condition: within 2% of the maximum.
                if (result.score - expected_max_score).abs() <= (expected_max_score * 0.02) {
                    sim.stop().unwrap();
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
}

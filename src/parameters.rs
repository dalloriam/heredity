use crate::FitnessEvaluator;

/// Parameters of a simulation.
pub struct Parameters<T: FitnessEvaluator> {
    /// Size of the population for every generation. (default = 100)
    ///
    /// Increasing this value might improve results (because of a larger gene pool), but will
    /// *drastically* increase time to convergence.
    pub population_size: usize,

    /// The length of the genetic code for all cells (default = 10)
    ///
    /// This value will most likely need to be changed in accordance with the problem you're trying to solve.
    pub genetic_code_length: usize,

    pub keep_threshold: f64,

    pub mutation_chance_percent: f64,

    /// How many generations between result samples (default = 1000)
    ///
    /// Since observing results are expensive (a result contains a _copy_ of the genetic code of the current best cell),
    /// this parameter allows fine-grained control on exactly when
    /// progress is observed. Lower values will decrease performance but increase observability, and vice-versa.
    pub emit_result_every: usize,

    /// The fitness evaluator to use for this simulation.
    pub fitness_evaluator: T,
}

impl<T> Parameters<T>
where
    T: FitnessEvaluator,
{
    pub fn new(evaluator: T) -> Parameters<T> {
        Parameters {
            population_size: 100,
            genetic_code_length: 10,
            keep_threshold: 0.5,
            mutation_chance_percent: 0.01,
            emit_result_every: 1000,
            fitness_evaluator: evaluator,
        }
    }

    #[must_use]
    pub fn with_population_size(mut self, population_size: usize) -> Self {
        self.population_size = population_size;
        self
    }

    #[must_use]
    pub fn with_genetic_code_length(mut self, length: usize) -> Self {
        self.genetic_code_length = length;
        self
    }

    #[must_use]
    pub fn with_keep_threshold(mut self, threshold: f64) -> Self {
        self.keep_threshold = threshold;
        self
    }

    #[must_use]
    pub fn with_mutation_chance_percent(mut self, mutation_chance: f64) -> Self {
        self.mutation_chance_percent = mutation_chance;
        self
    }
}

#[cfg(test)]
mod tests {

    use super::{FitnessEvaluator, Parameters};

    struct MockEvaluator;
    impl FitnessEvaluator for MockEvaluator {
        fn evaluate(&self, _code: &[u8]) -> f64 {
            0.0
        }
    }

    #[test]
    fn parameters_init() {
        let params = Parameters::new(MockEvaluator {});
        assert_eq!(params.genetic_code_length, 10);
        assert_eq!(params.population_size, 100);
        assert_eq!(params.keep_threshold, 0.5);
        assert_eq!(params.mutation_chance_percent, 0.01);
        assert_eq!(params.emit_result_every, 1000);
    }
}

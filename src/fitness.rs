/// Defines the evaluation heuristic for the current simulation.
pub trait FitnessEvaluator {
    /// Evaluate the provided genetic code, return a score.
    ///
    /// Higher = better.
    fn evaluate(&self, code: &Vec<u8>) -> f64;
}

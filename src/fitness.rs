pub trait FitnessEvaluator {
    fn evaluate(&self, code: &Vec<u8>) -> f64;
}

/// A SimResult is a single datapoint -- a snapshot of the best solution the simulation has found so far.
pub struct SimResult {
    /// The genetic code of the best cell.
    pub genes: Vec<u8>,

    /// The score of the best cell (as computed by the fitness evaluator).
    pub score: f64,
}

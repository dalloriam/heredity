/home/wdussault/.virtualenvs/core3/bin/python: No module named virtualfish
mod cell;
mod fitness;
mod simulation;

pub use fitness::FitnessEvaluator;
pub use simulation::Parameters as SimulationParameters;
pub use simulation::Simulation;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

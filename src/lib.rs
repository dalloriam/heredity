mod run;
mod cell;
mod fitness;
mod simulation;

pub use fitness::FitnessEvaluator as FitnessEvaluator;
pub use simulation::Parameters as SimulationParameters;
pub use simulation::Simulation as Simulation;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

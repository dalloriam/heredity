//! General library for genetic algorithm implementations.

use std::sync::mpsc;
use std::thread;

use snafu::{ensure, Snafu};

mod cell;
mod fitness;
mod parameters;
mod sim_result;
mod simulation;

pub use fitness::FitnessEvaluator;
pub use parameters::Parameters;
pub use sim_result::SimResult;
use simulation::Simulation;

#[derive(Debug, Snafu)]
pub enum SimulationError {
    AlreadyRunning,
    JoinError,
    NotRunning,
}

struct Handle {
    pub thread_handle: thread::JoinHandle<()>,
    pub stop_tx: mpsc::Sender<bool>,
}

/// Main simulation runner.
pub struct ThreadSim {
    handle: Option<Handle>,
}

impl ThreadSim {
    /// Creates a new empty simulation object.
    pub fn new() -> ThreadSim {
        ThreadSim { handle: None }
    }

    /// Starts a simulation thread using the provided parameters.
    ///
    /// `start` returns a result receiver that can be used to read result snapshots.
    pub fn start<T>(
        &mut self,
        params: Parameters<T>,
    ) -> Result<mpsc::Receiver<SimResult>, SimulationError>
    where
        T: 'static + FitnessEvaluator + Send + Sync,
    {
        ensure!(self.handle.is_none(), AlreadyRunning);
        let (result_tx, result_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        let sim = Simulation::new(params);
        let thread_handle =
            thread::spawn(move || ThreadSim::internal_sim_loop(sim, stop_rx, result_tx));

        self.handle = Some(Handle {
            stop_tx,
            thread_handle,
        });

        Ok(result_rx)
    }

    pub fn stop(&mut self) -> Result<(), SimulationError> {
        ensure!(self.handle.is_some(), NotRunning);
        let handle = self.handle.take().unwrap(); // Safe because of ensure.
        handle
            .stop_tx
            .send(true)
            .map_err(|_e| SimulationError::JoinError)?;
        handle
            .thread_handle
            .join()
            .map_err(|_e| SimulationError::JoinError)?;
        Ok(())
    }

    fn internal_sim_loop<T: FitnessEvaluator + Send + Sync>(
        mut sim: Simulation<T>,
        stop_rx: mpsc::Receiver<bool>,
        result_tx: mpsc::Sender<SimResult>,
    ) {
        sim.run(result_tx, stop_rx)
    }
}

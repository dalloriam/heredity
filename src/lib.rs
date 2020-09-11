//! General library for genetic algorithm implementations.

use std::sync::mpsc;
use std::thread;

use snafu::Snafu;

mod cell;
mod fitness;
mod parameters;
mod sim_result;
mod simulation;

pub use fitness::FitnessEvaluator;
pub use parameters::Parameters;
pub use sim_result::SimResult;
use simulation::Simulation;

/// Enumeration of possible errors returned by the simulation.
#[derive(Debug, Snafu)]
pub enum SimulationError {
    /// Returned when the simulation thread failed to stop.
    JoinError,
}

struct Handle {
    pub thread_handle: thread::JoinHandle<()>,
    pub stop_tx: mpsc::Sender<bool>,
}

/// Main simulation runner.
pub struct ThreadSim {
    handle: Handle,
    rx: mpsc::Receiver<SimResult>,
}

impl ThreadSim {
    /// Starts a simulation thread using the provided parameters.
    ///
    /// `start` returns a result receiver that can be used to read result snapshots.
    pub fn start<T>(params: Parameters<T>) -> ThreadSim
    where
        T: 'static + FitnessEvaluator + Send + Sync,
    {
        let (result_tx, result_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();

        let sim = Simulation::new(params);
        let thread_handle =
            thread::spawn(move || ThreadSim::internal_sim_loop(sim, stop_rx, result_tx));

        ThreadSim {
            handle: Handle {
                stop_tx,
                thread_handle,
            },
            rx: result_rx,
        }
    }

    /// Returns a reference to the simulation result receiver.
    pub fn results(&self) -> &mpsc::Receiver<SimResult> {
        &self.rx
    }

    /// Stops the simulation.
    pub fn stop(self) -> Result<(), SimulationError> {
        self.handle
            .stop_tx
            .send(true)
            .map_err(|_e| SimulationError::JoinError)?;
        self.handle
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

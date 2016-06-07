//! Example of a run using input files for the simulation and the system
//! This is the exact same simulation as the one in `binary.rs`

extern crate cymbalum;
use cymbalum::*;

fn main() {
    Logger::stdout();
    match input::read_config("data/simulation.toml") {
        Err(error) => println!("Error in input: {}", error),
        Ok(mut config) => {
            config.simulation.run(&mut config.system, config.nsteps);
        }
    }
}
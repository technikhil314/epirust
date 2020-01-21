#[macro_use]
extern crate serde_derive;

mod constants;

mod agent;
mod epidemiology_simulation;
mod allocation_map;
mod geography;
mod disease;
mod utils;
mod csv_service;

fn main() {
    println!("Executing for 10000 agents");
    const GRID_SIZE: i32 = 250;
    const NUMBER_OF_AGENTS: i32 = 10000;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_10000.csv";

    let mut epidemiology_simulation = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology_simulation.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
    println!("Done");
}

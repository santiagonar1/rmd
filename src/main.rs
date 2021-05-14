mod simulation;
mod grid;
mod particle;

use crate::simulation::Simulation;


fn main() {
    let filepath = "simulation.dat";
    let mut s = simulation::load_simulation(filepath);
    s.simulate();
}

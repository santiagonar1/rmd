use crate::grid::Grid;
use crate::particle::Particle;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Simulation {
    grid: Grid,
    delta_t: f64,
    t_end: f64,
}

pub fn load_simulation(filepath: &str) -> Simulation {
    let input = File::open(filepath).expect("Error opening file");
    let mut buffered = BufReader::new(input);
    let mut buf = String::new();

    buffered.read_line(&mut buf).unwrap();
    let mut buf = buf.replace("\n", "");
    let delta_t: f64 = buf.parse().unwrap();
    buf.clear();

    buffered.read_line(&mut buf).unwrap();
    let mut buf = buf.replace("\n", "");
    let t_end: f64 = buf.parse().unwrap();
    buf.clear();

    buffered.read_line(&mut buf).unwrap();
    let mut buf = buf.replace("\n", "");
    let num_particles: i64 = buf.parse().unwrap();
    buf.clear();

    let mut grid = Grid {
        particles: vec![],
    };

    for _i in 0..num_particles {
        buffered.read_line(&mut buf).unwrap(); // Whitespace
        buf.clear();

        // Mass
        buffered.read_line(&mut buf).unwrap();
        let mut buf = buf.replace("\n", "");
        let mass: f64 = buf.parse().unwrap();
        buf.clear();

        // Position
        buffered.read_line(&mut buf).unwrap();
        let mut buf = buf.replace("\n", "");
        let mut position: Vec<f64> = vec![];
        for p in buf.split(' ') {
            position.push(p.parse::<f64>().unwrap());
        }
        buf.clear();

        // Velocity
        buffered.read_line(&mut buf).unwrap();
        let mut buf = buf.replace("\n", "");
        let mut velocity: Vec<f64> = vec![];
        for v in buf.split(' ') {
            velocity.push(v.parse::<f64>().unwrap());
        }
        buf.clear();

        let mut particle = Particle::new();
        particle.mass = mass;
        particle.position = position;
        particle.velocity = velocity;
        grid.add_particle(particle);
    }

    Simulation {
        grid,
        delta_t,
        t_end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_simulation() {
        let filepath = "simulation.dat";
        let s = load_simulation(filepath);

        assert_eq!(s.delta_t, 0.015);
        assert_eq!(s.t_end, 468.5);

        assert_eq!(s.grid.particles[0].mass, 1.0);
        assert_eq!(s.grid.particles[0].position, vec![0.0, 0.0]);
        assert_eq!(s.grid.particles[0].velocity, vec![0.0, 0.0]);
        assert_eq!(s.grid.particles[1].mass, 3e-6);
        assert_eq!(s.grid.particles[1].position, vec![0.0, 1.0]);
        assert_eq!(s.grid.particles[1].velocity, vec![-1.0, 0.0]);
    }
}

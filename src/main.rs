struct Particle {
    mass: f64,
    position: Vec<f64>,
    velocity: Vec<f64>,
    force: Vec<f64>,
    force_old: Vec<f64>,
}

struct Grid {
    particles: Vec<Particle>,
}

fn calculate_force(p1: &Particle, p2: &Particle) -> Vec<f64> {
    let mut distance: f64 = 0.0;
    let num_dimensions = p1.position.len();

    for i in 0..num_dimensions {
        distance += (p1.position[i] - p2.position[i]).powi(2);
    }

    let f = (p1.mass * p2.mass) / (distance.sqrt() * distance);
    let mut force = p1.force.clone();

    for i in 0..num_dimensions {
        force[i] += f * (p2.position[i] - p1.position[i]);
    }

    force
}

impl Particle {
    fn update_position(&mut self, delta_t: f64) {
        let a = (delta_t * 0.5) / self.mass;
        for dim in 0..self.position.len() {
            self.position[dim] += delta_t * (self.velocity[dim] + a * self.force[dim]);
        }
    }

    fn store_old_force(&mut self) {
        for dim in 0..self.force.len() {
            self.force_old[dim] = self.force[dim];
        }
    }

    fn update_velocity(&mut self, delta_t: f64) {
        let a = (delta_t * 0.5) / self.mass;
        for dim in 0..self.velocity.len() {
            self.velocity[dim] += a * (self.force[dim] + self.force_old[dim]);
        }
    }
}

impl Grid {
    fn update_positions(&mut self, delta_t: f64) {
        for particle in self.particles.iter_mut() {
            particle.update_position(delta_t);
        }
    }

    fn store_old_forces(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.store_old_force();
        }
    }

    fn update_velocities(&mut self, delta_t: f64) {
        for particle in self.particles.iter_mut() {
            particle.update_velocity(delta_t);
        }
    }

    fn update_forces(&mut self) {
        for particle in self.particles.iter_mut() {
            for f in particle.force.iter_mut() {
                *f = 0.0;
            }
        }
        let num_particles = self.particles.len();
        for i in 0..num_particles {
            for j in 0..num_particles {
                if i != j {
                    self.particles[i].force =
                        calculate_force(&self.particles[i], &self.particles[j]);
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn can_create_particle() {
        let particle = Particle {
            mass: 2.1,
            position: vec![1.0, 2.0, 3.0],
            velocity: vec![4.0, 5.0, 6.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![7.0, 8.0, 9.0],
        };

        assert_eq!(particle.mass, 2.1);
        assert_eq!(particle.position, [1.0, 2.0, 3.0]);
        assert_eq!(particle.velocity, [4.0, 5.0, 6.0]);
        assert_eq!(particle.force, [7.0, 8.0, 9.0]);
        assert_eq!(particle.force_old, [7.0, 8.0, 9.0]);
    }

    #[test]
    fn can_update_particle_position() {
        let mut particle = Particle {
            mass: 20.0,
            position: vec![0.0, 1.0, 2.0],
            velocity: vec![4.0, 5.0, 6.0],
            force: vec![1.0, 10.0, 100.0],
            force_old: vec![0.0, 0.0, 0.0],
        };

        particle.update_position(0.5);
        assert_eq!(particle.position, [2.00625, 3.5625, 5.625]);
    }

    #[test]
    fn can_store_old_force() {
        let mut particle = Particle {
            mass: 20.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![4.0, 5.0, 6.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![0.0, 0.0, 0.0],
        };

        particle.store_old_force();
        assert_eq!(particle.force_old, [7.0, 8.0, 9.0]);
    }

    #[test]
    fn can_update_particle_velocity() {
        let mut particle = Particle {
            mass: 20.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![4.0, 5.0, 6.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        particle.update_velocity(0.5);
        assert_eq!(particle.velocity, [4.1, 5.125, 6.15]);
    }

    #[test]
    fn can_update_force() {
        let particle1 = Particle {
            mass: 20.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![4.0, 5.0, 6.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let particle2 = Particle {
            mass: 40.0,
            position: vec![1.0, 2.0, 3.0],
            velocity: vec![7.0, 8.0, 9.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let mut grid = Grid {
            particles: vec![particle1, particle2],
        };

        grid.update_forces();
        let correct_result: Vec<f64> = vec![-15.27207096642, -30.5441419328, -45.8162128993];
        for i in 0..correct_result.len() {
            assert_approx_eq!(grid.particles[1].force[i], correct_result[i]);
        }
    }
}

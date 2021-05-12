use crate::particle::Particle;

pub struct Grid {
    pub particles: Vec<Particle>,
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

impl Grid {
    pub fn update_positions(&mut self, delta_t: f64) {
        for particle in self.particles.iter_mut() {
            particle.update_position(delta_t);
        }
    }

    pub fn store_old_forces(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.store_old_force();
        }
    }

    pub fn update_velocities(&mut self, delta_t: f64) {
        for particle in self.particles.iter_mut() {
            particle.update_velocity(delta_t);
        }
    }

    pub fn update_forces(&mut self) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

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

    #[test]
    fn can_update_positions() {
        let position_first_particle = vec![0.0, 0.0, 0.0];
        let position_second_particle = vec![1.0, 2.0, 3.0];
        let delta_t = 0.5;

        let particle1 = Particle {
            mass: 20.0,
            position: position_first_particle.clone(),
            velocity: vec![4.0, 5.0, 6.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let particle2 = Particle {
            mass: 40.0,
            position: position_second_particle.clone(),
            velocity: vec![7.0, 8.0, 9.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let mut grid = Grid {
            particles: vec![particle1, particle2],
        };

        // TODO: Copy trate could remove the need of this
        let mut particle1 = Particle {
            mass: 20.0,
            position: position_first_particle.clone(),
            velocity: vec![4.0, 5.0, 6.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let mut particle2 = Particle {
            mass: 40.0,
            position: position_second_particle.clone(),
            velocity: vec![7.0, 8.0, 9.0],
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        grid.update_positions(delta_t);
        particle1.update_position(delta_t);
        particle2.update_position(delta_t);
        assert_eq!(grid.particles[0].position, particle1.position);
        assert_eq!(grid.particles[1].position, particle2.position);
    }

    #[test]
    fn can_update_velocities() {
        let velocity_first_particle = vec![4.0, 5.0, 6.0];
        let velocity_second_particle = vec![7.0, 8.0, 9.0];
        let delta_t = 0.5;

        let particle1 = Particle {
            mass: 20.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: velocity_first_particle.clone(),
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let particle2 = Particle {
            mass: 40.0,
            position: vec![1.0, 2.0, 3.0],
            velocity: velocity_second_particle.clone(),
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let mut grid = Grid {
            particles: vec![particle1, particle2],
        };

        // TODO: Copy trate could remove the need of this
        let mut particle1 = Particle {
            mass: 20.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: velocity_first_particle.clone(),
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        let mut particle2 = Particle {
            mass: 40.0,
            position: vec![1.0, 2.0, 3.0],
            velocity: velocity_second_particle.clone(),
            force: vec![7.0, 8.0, 9.0],
            force_old: vec![1.0, 2.0, 3.0],
        };

        grid.update_velocities(delta_t);
        particle1.update_velocity(delta_t);
        particle2.update_velocity(delta_t);
        assert_eq!(grid.particles[0].velocity, particle1.velocity);
        assert_eq!(grid.particles[1].velocity, particle2.velocity);
    }

    #[test]
    fn can_store_old_forces() {
        let force_first_particle = vec![4.0, 5.0, 6.0];
        let force_second_particle = vec![7.0, 8.0, 9.0];

        let particle1 = Particle {
            mass: 20.0,
            position: vec![0.0, 0.0, 0.0],
            velocity: vec![1.0, 2.0, 3.0],
            force: force_first_particle.clone(),
            force_old: vec![0.0, 0.0, 0.0],
        };

        let particle2 = Particle {
            mass: 40.0,
            position: vec![1.0, 2.0, 3.0],
            velocity: vec![1.0, 2.0, 3.0],
            force: force_second_particle.clone(),
            force_old: vec![0.0, 0.0, 0.0],
        };

        let mut grid = Grid {
            particles: vec![particle1, particle2],
        };

        grid.store_old_forces();
        assert_eq!(grid.particles[0].force_old, force_first_particle);
        assert_eq!(grid.particles[1].force_old, force_second_particle);
    }
}

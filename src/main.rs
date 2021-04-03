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

impl Particle {
    fn update_position(&mut self, delta_t: f64) {
        let a = (delta_t * 0.5) / self.mass;
        let mut dim = 0;
        for position in self.position.iter_mut() {
            *position += delta_t * (self.velocity[dim] + a * self.force[dim]);
            dim += 1;
        }
    }

    fn store_old_force(&mut self) {
        let mut dim = 0;
        for force in self.force.iter_mut() {
            self.force_old[dim] = *force;
            dim += 1;
        }
    }

    fn update_velocity(&mut self, delta_t: f64) {
        let a = (delta_t * 0.5) / self.mass;
        let mut dim = 0;
        for velocity in self.velocity.iter_mut() {
            *velocity += a * (self.force[dim] + self.force_old[dim]);
            dim += 1;
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
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

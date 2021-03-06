pub struct Particle {
    pub mass: f64,
    pub position: Vec<f64>,
    pub velocity: Vec<f64>,
    pub force: Vec<f64>,
    pub force_old: Vec<f64>,
}

impl Particle {
    pub fn new() -> Particle {
       Particle {
           mass: 0.0,
           position: vec![0.0, 0.0],
           velocity: vec![0.0, 0.0],
           force: vec![0.0, 0.0],
           force_old: vec![0.0, 0.0],
       }
    }
    pub fn update_position(&mut self, delta_t: f64) {
        let a = (delta_t * 0.5) / self.mass;
        for dim in 0..self.position.len() {
            self.position[dim] += delta_t * (self.velocity[dim] + a * self.force[dim]);
        }
    }

    pub fn store_old_force(&mut self) {
        for dim in 0..self.force.len() {
            self.force_old[dim] = self.force[dim];
        }
    }

    pub fn update_velocity(&mut self, delta_t: f64) {
        let a = (delta_t * 0.5) / self.mass;
        for dim in 0..self.velocity.len() {
            self.velocity[dim] += a * (self.force[dim] + self.force_old[dim]);
        }
    }
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

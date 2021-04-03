struct Particle {
    mass: f64,
    position: Vec<f64>,
    velocity: Vec<f64>,
    force: Vec<f64>,
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
        };

        assert_eq!(particle.mass, 2.1);
        assert_eq!(particle.position, [1.0, 2.0, 3.0]);
        assert_eq!(particle.velocity, [4.0, 5.0, 6.0]);
        assert_eq!(particle.force, [7.0, 8.0, 9.0]);
    }
}

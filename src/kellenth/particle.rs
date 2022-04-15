//! Holds the particle class and all its properties

#[allow(unused, dead_code)]
use crate::kellenth::core::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    /// Holds the position in world space of the particle
    pub position: Vector3,

    /// Holds the linear velocity of the particle in world space
    pub velocity: Vector3,

    /// Holds the acceleration of the particle.
    pub acceleration: Vector3,

    /// Holds the amount of damping applied in linear motion.
    /// Required for removing energy added through numerical instability. */
    pub damping: f64,

    /// Holds the amount of accumulated force to be applied
    /// in the next iteration of the simulation.
    /// The value will always be zero'd in the integration step
    accumulatedForce: Vector3,

    /// Holds the inverse mass of the particle.
    /// Holding the actual mass instead would slow calculations because
    /// we calculate the acceleration by using `1/mass` and that division
    /// computed every frame can cost a bit of performance.
    ///
    /// # To make the particle immovable set an `inverseMass` of zero.
    inverse_mass: f64
}

impl Particle {
    /// Constructor
    pub fn new(position: Vector3, velocity: Vector3, acceleration: Vector3,
                damping: f64) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            damping,
            accumulatedForce: Vector3 {x: 0., y: 0., z: 0.},
            inverse_mass: 0.0 }
    }
    
    /// Returns the inverse mass of the particle.
    pub fn get_inverse_mass(self) -> f64 {
        self.inverse_mass
    }

    /// Sets the inverse mass to given value.
    pub fn set_inverse_mass(mut self, inverse_mass: f64) {
        self.inverse_mass = inverse_mass;
    }

    /// Returns the mass of the particle.
    /// ### If the object is immovable, returns `f64::MAX`
    pub fn get_mass(self) -> f64 {
        if self.inverse_mass == 0. {
            return f64::MAX
        }
        return 1. / self.inverse_mass
    }

    /// Sets the mass of the object.
    /// It should not be zero.
    /// ### SMALL MASSES PRODUCE UNSTABLE RIGID BODIES UNDER SIMULATION
    pub fn set_mass(mut self, mass: f64) {
        assert!(mass != 0.);
        self.inverse_mass = 1. / mass;
    }

    /// Integrates the particle forward in time by the given amount.
    /// This function uses a Newton-Euler integration method, which
    /// is a linear aproximation of the correct integral.
    /// Recieves the duration between the last two frames as a parameter.
    /// ### IT MAY BE INNACURATE IN SOME CASES
    pub fn integrate(mut self, duration: f64) {
        assert!(duration > 0.);

        /// Update the linear position
        self.position.add_scaled_vector(self.velocity, duration);

        /// Work out the acceleration from the force.
        let resAcceleration = self.acceleration;
        // resAcceleration.add_scaled_vector(self.get_inverse_mass());

        /// Update linear velocity from the acceleration
        self.velocity.add_scaled_vector(resAcceleration, duration);

        /// Eliminate part of velocity with drag
        self.velocity *= powf64(self.damping, duration);
    }
}


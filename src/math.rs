use crate::geometry;

const GRAVITATIONAL_CONST: f64 = 6.6743e-11;

pub fn calculate_force(mass1: f64, mass2: f64, distance: geometry::Vector3) -> geometry::Vector3 {
    let force_magnitude = GRAVITATIONAL_CONST * mass1 * mass2 / distance.powi(2).magnitude();
    let force_direction = distance.norm();

    force_direction.scale(force_magnitude)
}

pub fn calculate_acceleration(force: geometry::Vector3, mass: f64) -> geometry::Vector3 {
    force.scale(1.0 / mass)
}
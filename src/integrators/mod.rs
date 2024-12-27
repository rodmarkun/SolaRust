use crate::physics;

pub enum IntegratorType {
    Euler,
    RK4(usize),
}

pub trait Integrator {
    fn step(&self, state: &mut physics::State, timestep: f64);
}

mod euler;
mod rk4;

pub use self::euler::EulerIntegrator;
pub use self::rk4::RK4Integrator;
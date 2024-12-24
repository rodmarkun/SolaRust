use crate::physics;
use crate::geometry::Vector3;

pub struct EulerIntegrator;

impl super::Integrator for EulerIntegrator {
    fn step(&self, state: &mut physics::State, timestep: f64) {
        let num_bodies = state.positions.len();
        let mut forces: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); num_bodies];
        
        // Calculate forces between all bodies
        for i in 0..num_bodies {
            for j in 0..num_bodies {
                if i != j {
                    let force = physics::calculate_force_euler(
                        state.masses[i],
                        state.masses[j],
                        state.positions[j].subtract(&state.positions[i])
                    );
                    forces[i] = forces[i].add(&force);
                }
            }
        }

        // Update positions and velocities
        for i in 0..num_bodies {
            let acc = physics::calculate_acceleration(forces[i].clone(), state.masses[i]);
            state.velocities[i] = state.velocities[i].add(&acc.scale(timestep));
            state.positions[i] = state.positions[i].add(&state.velocities[i].scale(timestep));
        }
    }
}
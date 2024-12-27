use crate::{geometry::Vector3, physics::{self}};

pub struct RK4Integrator {
    substeps: usize,
}

impl RK4Integrator {
    pub fn new(substeps: usize) -> Self {
        RK4Integrator {
            substeps: substeps.max(1)
        }
    }

    fn single_step(&self, state: &mut physics::State, timestep: f64) {
        let num_bodies = state.positions.len();

        let d1 = calculate_derivatives(state);
    
        let mut temp_state = state.advance_by_derivatives(d1.clone(), timestep / 2.0);
        let d2 = calculate_derivatives(&mut temp_state);
        
        let mut temp_state = state.advance_by_derivatives(d2.clone(), timestep / 2.0);
        let d3 = calculate_derivatives(&mut temp_state);
        
        let mut temp_state = state.advance_by_derivatives(d3.clone(), timestep);
        let d4 = calculate_derivatives(&mut temp_state);

        let derivatives = vec![d1, d2, d3, d4];
        let weights = [1.0/6.0, 1.0/3.0, 1.0/3.0, 1.0/6.0];

        for i in 0..num_bodies {
            let mut final_velocity = Vector3::new(0.0, 0.0, 0.0);
            let mut final_acceleration = Vector3::new(0.0, 0.0, 0.0);
            
            for (j, &weight) in weights.iter().enumerate() {
                final_velocity = final_velocity.add(&derivatives[j].0[i].scale(weight));
                final_acceleration = final_acceleration.add(&derivatives[j].1[i].scale(weight));
            }

            final_velocity = final_velocity.scale(timestep);
            final_acceleration = final_acceleration.scale(timestep);
            
            state.positions[i] = state.positions[i].add(&final_velocity);
            state.velocities[i] = state.velocities[i].add(&final_acceleration);
        }
    }
}

impl super::Integrator for RK4Integrator {
    fn step(&self, state: &mut physics::State, timestep: f64) {
        let substep_size = timestep / self.substeps as f64;
        
        for _ in 0..self.substeps {
            self.single_step(state, substep_size);
        }
    }
}

fn calculate_derivatives(state: &mut physics::State) -> (Vec<Vector3>, Vec<Vector3>) {
    let num_bodies = state.positions.len();
    let mut acceleration = vec![Vector3::new(0.0, 0.0, 0.0); num_bodies];
    let velocity = state.velocities.to_vec();

    for i in 0..num_bodies {
        for j in 0..num_bodies {
            if i != j {
                let force = physics::calculate_force_euler(state.masses[i], state.masses[j], state.positions[j].subtract(&state.positions[i]));
                acceleration[i] = acceleration[i].add(&force.scale(1.0 / state.masses[i]));
            }
        }
    }           
    (velocity, acceleration)                
}
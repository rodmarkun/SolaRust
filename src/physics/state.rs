use crate::geometry::Vector3;

#[derive(Clone)]
pub struct State {
    pub positions: Vec<Vector3>,
    pub velocities: Vec<Vector3>,
    pub masses: Vec<f64>
}

impl State {
    pub fn new(positions: Vec<Vector3>, velocities: Vec<Vector3>, masses: Vec<f64>) -> Self {
        State {positions, velocities, masses}
    }

    pub fn add(&self, other: State) -> Self {
        let summed_positions = self.positions.iter()
            .zip(other.positions.iter())
            .map(|(a,b)| a.add(b))
            .collect();

        let summed_velocities = self.velocities.iter()
            .zip(other.velocities.iter())
            .map(|(a,b)| a.add(b))
            .collect();

        State::new(summed_positions, summed_velocities, self.masses.clone())
    }

    pub fn scale(&self, scalar: f64) -> Self {
        let scaled_positions = self.positions.iter()
            .map(|a| a.scale(scalar))
            .collect();

        let scaled_velocities = self.velocities.iter()
            .map(|a| a.scale(scalar))
            .collect();

        State::new(scaled_positions, scaled_velocities, self.masses.clone())
    }

    pub fn advance_by_derivatives(&self, derivatives: (Vec<Vector3>, Vec<Vector3>), timestep: f64) -> State {
        let new_pos = self.positions.iter()
            .zip(derivatives.0.iter())
            .map(|(a, b)| a.add(&b.scale(timestep)))
            .collect();
        
        let new_vel = self.velocities.iter()
            .zip(derivatives.1.iter())
            .map(|(a, b)| a.add(&b.scale(timestep)))
            .collect();

        State::new(new_pos, new_vel, self.masses.clone())
    }
}
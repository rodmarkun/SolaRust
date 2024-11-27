use crate::geometry;

pub struct CelestialBody {
    pub name: String,
    pub body_type: BodyType,
    pub position: geometry::Vector3,
    pub km_radius: f64,
    pub mass: f64,
    pub velocity: geometry::Vector3,
    pub color: [f32; 3]
}

pub enum BodyType {
    Star,
    Planet,
    Moon,
    Satellite
}

impl CelestialBody {
    pub fn new(name: String, body_type: BodyType, position: geometry::Vector3, km_radius: f64, mass: f64, velocity: geometry::Vector3, color: [f32; 3]) -> CelestialBody {
        CelestialBody {name, body_type, position, km_radius, mass, velocity, color}
    }

    pub fn calculate_display_size(&self) -> f32 {
        ((self.km_radius.ln() - 10.0) / 2.0) as f32
    }
}
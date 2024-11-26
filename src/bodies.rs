use crate::geometry;

pub struct CelestialBody {
    pub name: String,
    pub position: geometry::Vector3,
    pub km_radius: f64,
    pub color: [f32; 3]
}

impl CelestialBody {
    pub fn new(name: String, position: geometry::Vector3, km_radius: f64, color: [f32; 3]) -> CelestialBody {
        CelestialBody {name, position, km_radius, color}
    }
}
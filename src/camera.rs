use crate::geometry;

pub struct Camera {
    position: geometry::Vector3,
    target: geometry::Vector3,
    up: geometry::Vector3,
    fov: f32,
    aspect: f32
}

impl Camera {
    pub fn new(initial_position: &geometry::Vector3, initial_target: &geometry::Vector3) -> Self {
        Camera {position: initial_position.clone(),
                target: initial_target.clone(),
                up: geometry::Vector3::new(0.0, 1.0, 0.0),
                fov: 120.0,
                aspect: 16.0 / 9.0}
    }
}
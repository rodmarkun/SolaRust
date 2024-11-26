use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{Point3, Translation3};
use crate::bodies::CelestialBody;

const DISPLAY_SCALE: f32 = 1e-6; 

pub struct Renderer {
    window: Window,
    celestial_bodies: Vec<SceneNode>,
}

impl Renderer {
    pub fn new() -> Self {
        let window = Window::new("Solar System");
        Renderer {
            window,
            celestial_bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: &CelestialBody) {
        let mut sphere = self.window.add_sphere(body.km_radius as f32 * DISPLAY_SCALE);
        sphere.set_color(body.color[0], body.color[1], body.color[2]);
        
        let scaled_pos = body.position.scale(DISPLAY_SCALE.into());
        sphere.set_local_translation(Translation3::new(
            scaled_pos.x as f32, 
            scaled_pos.y as f32, 
            scaled_pos.z as f32
        ));
        
        self.celestial_bodies.push(sphere);
    }

    pub fn render_loop(&mut self) {
        let mut camera = ArcBall::new(Point3::new(0.0, 0.0, 10.0), Point3::origin());
        self.window.set_light(Light::StickToCamera);

        while self.window.render_with_camera(&mut camera) {
            // Physics updates here
        }
    }
}
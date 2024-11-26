use satellite::geometry;
use satellite::camera;
use satellite::renderer;
use satellite::bodies;

fn main() {
    let mut renderer = renderer::Renderer::new();

    let sun = bodies::CelestialBody {
        position: geometry::Vector3::new(0.0, 0.0, 0.0),
        km_radius: 696_340.0,  // Sun's radius in km
        color: [1.0, 1.0, 0.0],  // Yellow
        name: String::from("Sun"),
    };

    renderer.add_body(&sun);
    renderer.render_loop();
}
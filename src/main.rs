use satellite::renderer;
use satellite::solar_system::SolarSystem;

fn main() {
    let mut renderer = renderer::Renderer::new();
    let mut solar_system = SolarSystem::initialize_standard();

    for body in solar_system.get_bodies() {
        renderer.add_body(body);
    }

    // Start simulation
    renderer.render_loop(&mut solar_system);
}
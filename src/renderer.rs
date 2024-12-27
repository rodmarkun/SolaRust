use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::event::{Key, Action};
use kiss3d::resource::MaterialManager;
use std::time::SystemTime;
use rand::Rng;
use crate::{solar_system, body::{self, BodyType}};

const DISPLAY_SCALE: f32 = 1e-9;
const NUM_STARS: usize = 1000;

pub struct BodyVisuals {
    main_body: SceneNode,
    effects: Vec<SceneNode>,
    atmosphere: Option<SceneNode>,
    rotation_speed: f32,
    trail: Vec<Point3<f32>>,
}

pub struct Starfield {
    stars: Vec<Point3<f32>>,
    brightness: Vec<f32>,
}

pub struct Renderer {
    window: Window,
    bodies: Vec<BodyVisuals>,
    starfield: Starfield,
    max_trail_length: usize,
    trail_interpolation_points: usize,
}

impl Renderer {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::with_capacity(NUM_STARS);
        let mut brightness = Vec::with_capacity(NUM_STARS);
        
        // Generate random stars in a sphere
        for _ in 0..NUM_STARS {
            let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let phi = (rng.gen::<f32>() * 2.0 - 1.0).acos();
            let r = rng.gen::<f32>().powf(1.0/3.0) * 50000.0;
            
            stars.push(Point3::new(
                r * phi.sin() * theta.cos(),
                r * phi.sin() * theta.sin(),
                r * phi.cos()
            ));
            
            brightness.push(rng.gen::<f32>() * 0.5 + 0.5);
        }

        Renderer {
            window: Window::new("Solar System"),
            bodies: Vec::new(),
            starfield: Starfield { stars, brightness },
            max_trail_length: 3000,
            trail_interpolation_points: 30,
        }
    }

    fn create_star_visuals(&mut self, body: &body::CelestialBody) -> BodyVisuals {
        let mut main_body = self.window.add_sphere(body.calculate_display_size());
        main_body.set_color(1.0, 1.0, 0.7);
        
        let mut effects = Vec::new();
        
        // Create corona layers with different sizes
        for i in 0..3 {
            let size = 4.0 + i as f32 * 0.5;
            let mut corona = self.window.add_sphere(size);
            corona.set_color(1.0, 0.8 - i as f32 * 0.1, 0.0);
            // Set material properties for corona glow
            let material = MaterialManager::new().get_default();
            corona.set_material(material);
            effects.push(corona);
        }

        BodyVisuals {
            main_body,
            effects,
            atmosphere: None,
            rotation_speed: 0.001,
            trail: Vec::new(),
        }
    }

    fn create_planet_visuals(&mut self, body: &body::CelestialBody) -> BodyVisuals {
        let mut main_body = self.window.add_sphere(body.calculate_display_size());
        main_body.set_color(body.color[0], body.color[1], body.color[2]);
        
        // Add surface material properties
        let surface_material = MaterialManager::new().get_default();
        main_body.set_material(surface_material);

        // Add atmosphere for planets
        let atmosphere = if body.body_type == BodyType::Planet {
            let mut atm = self.window.add_sphere(body.calculate_display_size() * 1.05);
            let atm_material = MaterialManager::new().get_default();
            atm.set_material(atm_material);
            atm.set_color(0.6, 0.8, 1.0);
            Some(atm)
        } else {
            None
        };

        let mut rng = rand::thread_rng();
        let rotation_speed = 0.005 + rng.gen::<f32>() * 0.002;

        BodyVisuals {
            main_body,
            effects: Vec::new(),
            atmosphere,
            rotation_speed,
            trail: Vec::new(),
        }
    }

    fn render_starfield(&mut self) {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f32();

        for (i, star) in self.starfield.stars.iter().enumerate() {
            let twinkle = (time * 2.0 + i as f32).sin() * 0.2 + 0.8;
            let brightness = self.starfield.brightness[i] * twinkle;
            
            self.window.draw_point(
                star,
                &Point3::new(brightness, brightness, brightness)
            );
        }
    }

    pub fn add_body(&mut self, body: &body::CelestialBody) {
        let mut visuals = match body.body_type {
            BodyType::Star => self.create_star_visuals(body),
            BodyType::Planet => self.create_planet_visuals(body),
            BodyType::Moon => self.create_planet_visuals(body),
            BodyType::Satellite => self.create_planet_visuals(body),
        };

        let scaled_pos = body.position.scale(DISPLAY_SCALE.into());
        let translation = Translation3::new(
            scaled_pos.x as f32,
            scaled_pos.y as f32,
            scaled_pos.z as f32
        );

        visuals.main_body.set_local_translation(translation);
        for effect in &mut visuals.effects {
            effect.set_local_translation(translation);
        }

        self.bodies.push(visuals);
    }

    fn handle_camera_input(&mut self, camera: &mut ArcBall, bodies: &Vec<body::CelestialBody>) {
        let movement_speed = 10.0;
        let zoom_speed = 1.05;
    
        let current_at = camera.at();
        let current_yaw = camera.yaw();
        let current_pitch = camera.pitch();
        let current_dist = camera.dist();
    
        let mut new_at = current_at;
    
        // W/S for up/down (Y axis)
        if self.window.get_key(Key::W) == Action::Press {
            new_at.y += movement_speed;
        }
        if self.window.get_key(Key::S) == Action::Press {
            new_at.y -= movement_speed;
        }
    
        // A/D for left/right (X axis)
        if self.window.get_key(Key::A) == Action::Press {
            new_at.x -= movement_speed;
        }
        if self.window.get_key(Key::D) == Action::Press {
            new_at.x += movement_speed;
        }
    
        // Q/E for zoom
        if self.window.get_key(Key::Q) == Action::Press {
            camera.set_dist(current_dist / zoom_speed);  // Zoom in
        }
        if self.window.get_key(Key::E) == Action::Press {
            camera.set_dist(current_dist * zoom_speed);  // Zoom out
        }
    
        if new_at != current_at {
            camera.set_at(new_at);
            camera.set_yaw(current_yaw);
            camera.set_pitch(current_pitch);
            camera.set_dist(current_dist);
        }
    
        // Focus on Earth
        if self.window.get_key(Key::F) == Action::Press {
            if let Some(earth) = bodies.iter().find(|b| b.name == "Earth") {
                let scaled_pos = earth.position.scale(DISPLAY_SCALE.into());
                camera.look_at(
                    Point3::new(
                        scaled_pos.x as f32 + 10.0,
                        scaled_pos.y as f32 + 10.0,
                        scaled_pos.z as f32 + 10.0
                    ),
                    Point3::new(scaled_pos.x as f32, scaled_pos.y as f32, scaled_pos.z as f32)
                );
            }
        }
    
        // Reset view
        if self.window.get_key(Key::R) == Action::Press {
            camera.look_at(
                Point3::new(0.0, 150.0, 150.0),
                Point3::origin()
            );
        }
    }

    pub fn update_positions(&mut self, bodies: &Vec<body::CelestialBody>) {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f32();

        // Render starfield first
        self.render_starfield();

        let mut trails_to_draw: Vec<(Point3<f32>, Point3<f32>, [f32; 3], f32)> = Vec::new();

        for (visuals, body) in self.bodies.iter_mut().zip(bodies.iter()) {
            let scaled_pos = body.position.scale(DISPLAY_SCALE.into());
            let point = Point3::new(
                scaled_pos.x as f32,
                scaled_pos.y as f32,
                scaled_pos.z as f32
            );
            
            // Update rotation
            let rotation = UnitQuaternion::from_axis_angle(
                &Vector3::y_axis(),
                visuals.rotation_speed * time
            );
            visuals.main_body.set_local_rotation(rotation);
            
            match body.body_type {
                BodyType::Star => {
                    let pulse = (time * 1.5).sin() * 0.1 + 0.9;
                    visuals.main_body.set_local_translation(Translation3::from(point.coords));
                    
                    // Animate corona
                    for (i, effect) in visuals.effects.iter_mut().enumerate() {
                        let phase = time + i as f32 * 0.5;
                        let scale = pulse * (1.0 + (phase * 2.0).sin() * 0.1);
                        effect.set_local_translation(Translation3::from(point.coords));
                        effect.set_local_scale(scale, scale, scale);
                        effect.set_color(1.0, pulse * 0.8, 0.0);
                    }
                },
                _ => {
                    visuals.main_body.set_local_translation(Translation3::from(point.coords));
                    
                    // Update atmosphere if present
                    if let Some(ref mut atmosphere) = visuals.atmosphere {
                        atmosphere.set_local_translation(Translation3::from(point.coords));
                        atmosphere.set_local_rotation(rotation);
                        
                        let atm_pulse = (time + body.position.magnitude() as f32).sin() * 0.02 + 1.0;
                        atmosphere.set_local_scale(atm_pulse, atm_pulse, atm_pulse);
                    }
                    
                    // Handle trail points
                    if visuals.trail.is_empty() {
                        // If trail is empty, add the first point
                        visuals.trail.push(point);
                    } else {
                        let last_point = *visuals.trail.last().unwrap();
                        
                        // Calculate velocity-based interpolation points
                        let velocity_magnitude = (point - last_point).norm();
                        let adaptive_points = ((self.trail_interpolation_points as f32 * velocity_magnitude.min(2.0)) as usize).max(1);
                        
                        // Create interpolated points
                        let mut new_points = Vec::new();
                        for i in 1..=adaptive_points {
                            let t = i as f32 / adaptive_points as f32;
                            let interpolated_point = Point3::new(
                                last_point.x + (point.x - last_point.x) * t,
                                last_point.y + (point.y - last_point.y) * t,
                                last_point.z + (point.z - last_point.z) * t
                            );
                            new_points.push(interpolated_point);
                        }
                        
                        // Add new points to trail
                        visuals.trail.extend(new_points);
                        
                        // Maintain maximum trail length
                        while visuals.trail.len() > self.max_trail_length {
                            visuals.trail.remove(0);
                        }
                    }
                
                    // Draw trail segments
                    if visuals.trail.len() > 1 {
                        for i in 0..visuals.trail.len()-1 {
                            let fade = (i as f32 / visuals.trail.len() as f32).powf(0.5);  // Square root for more gradual fade
                            trails_to_draw.push((
                                visuals.trail[i],
                                visuals.trail[i+1],
                                body.color,
                                fade
                            ));
                        }
                    }
                }
            }
        }

        // Draw trails with improved alpha blending
        for (start, end, color, fade) in trails_to_draw {
            self.window.draw_line(
                &start,
                &end,
                &Point3::new(
                    color[0] * fade,
                    color[1] * fade,
                    color[2] * fade
                )
            );
        }
    }

    pub fn render_loop(&mut self, solar_system: &mut solar_system::SolarSystem) {
        let mut camera = ArcBall::new_with_frustrum(
            120.0,
            0.1,
            1e7,
            Point3::new(0.0, 150.0, 150.0), 
            Point3::origin()
        );
        self.window.set_light(Light::StickToCamera);
    
        while self.window.render_with_camera(&mut camera) {
            // Handle camera movement
            self.handle_camera_input(&mut camera, solar_system.get_bodies());

            if self.window.get_key(Key::Up) == Action::Press {
                solar_system.timestep = solar_system.timestep * 1.1;
            }
            if self.window.get_key(Key::Down) == Action::Press {
                solar_system.timestep = solar_system.timestep * 0.9;
            }

            solar_system.update();
            self.update_positions(solar_system.get_bodies());
        }
    }
}
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::camera::ArcBall;
use kiss3d::nalgebra::{Point3, Translation3};
use kiss3d::event::{Key, Action};
use std::time::SystemTime;
use crate::{solar_system, body::{self, BodyType}};

const DISPLAY_SCALE: f32 = 1e-9;  

pub struct BodyVisuals {
    main_body: SceneNode,
    effects: Vec<SceneNode>,
    trail: Vec<Point3<f32>>
}

pub struct Renderer {
    window: Window,
    bodies: Vec<BodyVisuals>,
    max_trail_length: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            window: Window::new("Solar System"),
            bodies: Vec::new(),
            max_trail_length: 1000,
        }
    }

    fn create_star_visuals(&mut self, body: &body::CelestialBody) -> BodyVisuals {
        let mut main_body = self.window.add_sphere(body.calculate_display_size());
        main_body.set_color(1.0, 1.0, 0.7);  // Bright white-yellow
        let mut effects = Vec::new();
        
        let mut outer = self.window.add_sphere(4.0);
        outer.set_color(1.0, 0.8, 0.0);
        
        let mut inner = self.window.add_sphere(3.5);
        inner.set_color(1.0, 0.9, 0.2);
        
        effects.push(outer);
        effects.push(inner);
    
        BodyVisuals {
            main_body,
            effects,
            trail: Vec::new()
        }
    }

    fn create_planet_visuals(&mut self, body: &body::CelestialBody) -> BodyVisuals {
        let mut main_body = self.window.add_sphere(body.calculate_display_size());
        main_body.set_color(body.color[0], body.color[1], body.color[2]);

        BodyVisuals {
            main_body,
            effects: Vec::new(),
            trail: Vec::new()
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
        let mut trails_to_draw: Vec<(Point3<f32>, Point3<f32>, [f32; 3], f32)> = Vec::new();

        for (visuals, body) in self.bodies.iter_mut().zip(bodies.iter()) {
            let scaled_pos = body.position.scale(DISPLAY_SCALE.into());
            let point = Point3::new(
                scaled_pos.x as f32,
                scaled_pos.y as f32,
                scaled_pos.z as f32
            );
            
            match body.body_type {
                BodyType::Star => {
                    let time = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f32();
                    
                    let pulse = (time * 1.5).sin() * 0.1 + 0.9;
                    
                    visuals.main_body.set_local_translation(Translation3::from(point.coords));
                    
                    for effect in visuals.effects.iter_mut() {
                        effect.set_local_translation(Translation3::from(point.coords));
                        effect.set_color(1.0, pulse * 0.8, 0.0);
                    }
                },
                _ => {
                    visuals.main_body.set_local_translation(Translation3::from(point.coords));
                    
                    visuals.trail.push(point);
                    if visuals.trail.len() > self.max_trail_length {
                        visuals.trail.remove(0);
                    }

                    if visuals.trail.len() > 1 {
                        for i in 0..visuals.trail.len()-1 {
                            let fade = i as f32 / visuals.trail.len() as f32;
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
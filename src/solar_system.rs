use crate::body::{self, BodyType};
use crate::geometry;
use crate::integrators::{self, Integrator};
use crate::physics;

pub struct SimulationParameters {
    pub time_multiplier: f64,
}

impl SimulationParameters {
    pub fn new() -> Self {
        SimulationParameters {
            time_multiplier: 1.0
        }
    }
}

pub struct SolarSystem {
    pub bodies: Vec<body::CelestialBody>, 
    pub timestep: f64,
    integrator_type: integrators::IntegratorType,
}

impl SolarSystem {
    pub fn new(timestep: f64, integrator: integrators::IntegratorType) -> Self {
        SolarSystem {
            bodies: Vec::new(),
            timestep,
            integrator_type: integrator
        }
    }

    pub fn add_body(&mut self, body: body::CelestialBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {

        let positions: Vec<geometry::Vector3> = self.bodies.iter()
        .map(|body| body.position.clone())
        .collect();
        
        let velocities: Vec<geometry::Vector3> = self.bodies.iter()
            .map(|body| body.velocity.clone())
            .collect();
            
        let masses: Vec<f64> = self.bodies.iter()
            .map(|body| body.mass)
            .collect();
            
        let mut state = physics::State::new(positions, velocities, masses);

        match self.integrator_type {
            integrators::IntegratorType::Euler => {
                integrators::EulerIntegrator.step(&mut state, self.timestep);
            },
            integrators::IntegratorType::RK4 => {
                integrators::RK4Integrator.step(&mut state, self.timestep);
            }
        }
        
        // Update our bodies with the new state
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.position = state.positions[i].clone();
            body.velocity = state.velocities[i].clone();
        }

    }

    pub fn get_bodies(&self) -> &Vec<body::CelestialBody> {
        &self.bodies
    }

    pub fn initialize_standard() -> Self {
        let mut system = SolarSystem::new(3600.0, integrators::IntegratorType::RK4);  // 1 hour timestep

        // Sun
        system.add_body(body::CelestialBody::new(
            String::from("Sun"),
            BodyType::Star,
            geometry::Vector3::new(0.0, 0.0, 0.0),
            696_340.0,                // radius in km
            1.989e30,                 // mass in kg
            geometry::Vector3::new(0.0, 0.0, 0.0),
            [1.0, 1.0, 0.0]          // yellow
        ));

        // Mercury
        system.add_body(body::CelestialBody::new(
            String::from("Mercury"),
            BodyType::Planet,
            geometry::Vector3::new(57.9e9, 0.0, 0.0),
            2_439.7,
            3.285e23,
            geometry::Vector3::new(0.0, 47360.0, 0.0),
            [0.7, 0.7, 0.7]          // grey
        ));

        // Venus
        system.add_body(body::CelestialBody::new(
            String::from("Venus"),
            BodyType::Planet,
            geometry::Vector3::new(108.2e9, 0.0, 0.0),
            6_051.8,
            4.867e24,
            geometry::Vector3::new(0.0, 35020.0, 0.0),
            [0.9, 0.7, 0.5]          // pale yellow
        ));

        // Earth
        system.add_body(body::CelestialBody::new(
            String::from("Earth"),
            BodyType::Planet,
            geometry::Vector3::new(149.6e9, 0.0, 0.0),
            6_371.0,
            5.972e24,
            geometry::Vector3::new(0.0, 29780.0, 0.0),
            [0.2, 0.5, 1.0]          // blue
        ));

        // Mars
        system.add_body(body::CelestialBody::new(
            String::from("Mars"),
            BodyType::Planet,
            geometry::Vector3::new(227.9e9, 0.0, 0.0),
            3_389.5,
            6.39e23,
            geometry::Vector3::new(0.0, 24080.0, 0.0),
            [1.0, 0.3, 0.0]          // red
        ));

        // Jupiter
        system.add_body(body::CelestialBody::new(
            String::from("Jupiter"),
            BodyType::Planet,
            geometry::Vector3::new(778.5e9, 0.0, 0.0),
            69_911.0,
            1.898e27,
            geometry::Vector3::new(0.0, 13070.0, 0.0),
            [0.8, 0.6, 0.4]          // orange-brown
        ));

        // Saturn
        system.add_body(body::CelestialBody::new(
            String::from("Saturn"),
            BodyType::Planet,
            geometry::Vector3::new(1.434e12, 0.0, 0.0),
            58_232.0,
            5.683e26,
            geometry::Vector3::new(0.0, 9680.0, 0.0),
            [0.9, 0.8, 0.5]          // pale gold
        ));

        // Uranus
        system.add_body(body::CelestialBody::new(
            String::from("Uranus"),
            BodyType::Planet,
            geometry::Vector3::new(2.871e12, 0.0, 0.0),
            25_362.0,
            8.681e25,
            geometry::Vector3::new(0.0, 6800.0, 0.0),
            [0.5, 0.8, 0.9]          // pale blue
        ));

        // Neptune
        system.add_body(body::CelestialBody::new(
            String::from("Neptune"),
            BodyType::Planet,
            geometry::Vector3::new(4.495e12, 0.0, 0.0),
            24_622.0,
            1.024e26,
            geometry::Vector3::new(0.0, 5430.0, 0.0),
            [0.0, 0.0, 0.8]          // deep blue
        ));

        system
    }
}
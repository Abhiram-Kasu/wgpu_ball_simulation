use winit::event_loop::EventLoop;

use crate::app::App;

mod app;

/// Configuration for the ball simulation
pub struct SimulationConfig {
    pub window_width: u32,
    pub window_height: u32,
    pub ball_count: usize,
    pub ball_radius: f32,
    pub ball_color: Option<[f32; 4]>,
    pub gravity: [f32; 2],
    pub restitution: f32,
    pub damping: f32,
    pub max_velocity: f32,
    pub dt: f32,
    pub collision_softness: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            window_width: 1280,
            window_height: 720,
            ball_count: 1000,
            ball_radius: 3.0,
            ball_color: None, // Random colors
            gravity: [0.0, 0.5],
            restitution: 0.7,
            damping: 0.98,
            max_velocity: 10.0,
            dt: 0.016,
            collision_softness: 0.5,
        }
    }
}

fn main() {
    // Create your custom configuration here
    let config = SimulationConfig {
        window_width: 1600,
        window_height: 900,
        ball_count: 100000 / 5,
        ball_radius: 2.5 * 2.0,
        ball_color: Some([0.0, 1.0, 1.0, 0.5]), // Random colors, or use Some([r, g, b, a])
        gravity: [0.0, 10.0],
        restitution: 0.5,
        damping: 0.99,
        max_velocity: 30.0,
        dt: 0.016,               // ~60 FPS timestep
        collision_softness: 0.5, // How much balls can compress (0.0 = hard, 1.0 = very soft)
    };

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    let mut app = App::new(config);

    event_loop.run_app(&mut app).unwrap();
}

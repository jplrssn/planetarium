mod utils;

use rand::distributions;
use rand::distributions::Distribution;

use wasm_bindgen::prelude::*;
use web_time as time;

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
pub fn setup() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }
}

pub struct Planet {
    position: Vec2,
    velocity: Vec2,
    radius: f64,
}

#[wasm_bindgen]
pub struct State {
    planets: Vec<Planet>,
    last_frame: time::Instant,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new() -> State {
        State {
            planets: vec![],
            last_frame: time::Instant::now(),
        }
    }
}

static WIDTH: f64 = 4000.0;
static HEIGHT: f64 = 4000.0;
static NUM_PLANETS: u32 = 600;

#[wasm_bindgen]
pub fn init_state() -> State {
    // Generate random planets
    let max_velocity = 150.0;
    let min_velocity = -150.0;

    let max_radius = 200.0;
    let min_radius = 1.0;

    let dist_position_x = distributions::Uniform::new_inclusive(0.0, WIDTH);
    let dist_position_y = distributions::Uniform::new_inclusive(0.0, HEIGHT);
    let dist_velocity = distributions::Uniform::new_inclusive(min_velocity, max_velocity);
    let dist_radius = distributions::Uniform::new_inclusive(min_radius, max_radius);

    let mut rng = rand::thread_rng();

    let mut state = State::new();

    for _ in 0..NUM_PLANETS {
        let radius = 400.0 / dist_radius.sample(&mut rng);
        let velocity = Vec2::new(
            dist_velocity.sample(&mut rng),
            dist_velocity.sample(&mut rng),
        ) / (radius * 0.5);
        let position = Vec2::new(
            dist_position_x.sample(&mut rng),
            dist_position_y.sample(&mut rng),
        );

        state.planets.push(Planet {
            position: position,
            velocity: velocity,
            radius: radius,
        });
    }
    state
}

fn draw_planet(ctx: &web_sys::CanvasRenderingContext2d, radius: f64, pos: Vec2) {
    ctx.begin_path();
    let _ = ctx.arc(pos.x, pos.y, radius, 0.0, 2.0 * std::f64::consts::PI);
    ctx.fill();
    ctx.close_path();
}

#[wasm_bindgen]
pub fn render(state: &mut State, ctx: web_sys::CanvasRenderingContext2d) {
    let delta = state.last_frame.elapsed().as_secs_f64();
    state.last_frame = time::Instant::now();

    // update positions
    for planet in &mut state.planets {
        let offset = Vec2::new(planet.velocity.x * delta, planet.velocity.y * delta);
        planet.position += offset;

        // Wrap around edges
        if planet.position.x > WIDTH + planet.radius {
            planet.position.x -= WIDTH + 2.0 * planet.radius;
        }

        if planet.position.x < 0.0 - planet.radius {
            planet.position.x += WIDTH + 2.0 * planet.radius;
        }

        if planet.position.y > HEIGHT + planet.radius {
            planet.position.y -= HEIGHT + 2.0 * planet.radius;
        }

        if planet.position.y < 0.0 - planet.radius {
            planet.position.y += HEIGHT + 2.0 * planet.radius;
        }
    }

    // Draw planets
    for planet in &state.planets {
        draw_planet(&ctx, planet.radius, planet.position);
    }
}

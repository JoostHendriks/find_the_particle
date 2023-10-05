
use ggez::{
    event::{self, MouseButton},
    glam::*,
    graphics::{self, Color},
    Context, 
    GameResult,
    GameError,
    ContextBuilder,
    conf::WindowSetup
};
use std::{env, path};

extern crate rand;

use rand::Rng;

struct Particle {
    color: f32,
    size: f32,
    opacity: f32,
}

struct MainState {
    dt: std::time::Duration,
    fps: f64,
    image1: graphics::Image,
    coords: Vec2,
    mouse_down: bool,
    particles_found: i32,
    hit_spin: bool,
    spin_counter: i32,
    particle: Particle,
}

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;

const X_MIN: f32 = WIDTH/2.0 - 45.0;
const X_MAX: f32 = WIDTH/2.0 + 45.0;
const Y_MIN: f32 = HEIGHT/2.0 - 130.0;
const Y_MAX: f32 = HEIGHT/2.0 + 185.0;


impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::from_path(ctx, "/syringe.png")?;


        let s = MainState { 
            dt: std::time::Duration::new(0, 0), 
            fps: 0.0,
            image1,
            coords: Vec2::new(rand_coord_gen_low(X_MIN, X_MAX), Y_MAX),
            mouse_down: false,
            particles_found: 0,
            hit_spin: false,
            spin_counter: 0,
            particle: Particle {
                color: rand_double(),
                size: rand_double(),
                opacity: rand_double(),
                },
            };
        Ok(s)
    }
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        self.fps = ctx.time.fps();

        if self.hit_spin {
            self.coords = rand_coord_gen(X_MIN, X_MAX, Y_MIN, Y_MAX);
            self.spin_counter += 1;
        }

        if self.spin_counter >= 100 {
            self.coords.y = self.coords.y + 1.0;
            self.hit_spin = false;
            if self.coords.y >= Y_MAX - 2.0 {
                self.spin_counter = 0;
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        let rectangle = graphics::Mesh::new_rounded_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-200.0, -350.0, 400.0, 700.0),
            25.0,
            Color::WHITE,
        )?;
        canvas.draw(&rectangle, Vec2::new(WIDTH/2.0,HEIGHT/2.0));

        canvas.draw(
            graphics::Text::new(format!("Frame rate = {}ms {}Hz", self.dt.as_millis(), self.fps.round()))
                .set_scale(12.),
                Vec2::new(WIDTH-200.0,HEIGHT-20.0),
        );

        canvas.draw(&self.image1, Vec2::new(WIDTH/2.0-180.0, HEIGHT/2.0-350.0));

        let particle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            self.particle.size / 4.0 + 1.0,
            1.0,
            Color::from([self.particle.color, self.particle.color, self.particle.color, self.particle.opacity/2.0+0.3]),
        )?;
        canvas.draw(&particle, self.coords);

        // Draw score
        canvas.draw(
            graphics::Text::new(format!("Particles found {}", self.particles_found))
                .set_scale(24.),
                Vec2::new(5.0, 5.0),
        );
        // Draw button
        let rectangle_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(-292.0, -302.0, 57.0, 28.0),
            Color::WHITE,
        )?;
        canvas.draw(&rectangle_button, Vec2::new(WIDTH/2.0,HEIGHT/2.0));

        canvas.draw(
            graphics::Text::new(format!("Spin"))
                .set_scale(24.),
                Vec2::new(WIDTH/2.0-290.0, HEIGHT/2.0-300.0),
        );
        // Draw text
        let mut text: &str = " ";
        match self.particles_found {
            1 => text = "Good busy!",
            5 => text = "5 particles found already!",
            10 => text = "10 found! I would ask Joren for a raise",
            _ => (),
        }
        canvas.draw( graphics::Text::new(format!("{}", text))
            .set_scale(24.),
            Vec2::new(WIDTH/2.0-200.0, HEIGHT/2.0+360.0),
        );

        canvas.finish(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        self.mouse_down = true;
        let margin_particle_x = 5.0;
        let margin_particle_y = 5.0;
        let hit_particle = check_coords(self.coords.x, self.coords.y, x, y, margin_particle_x, margin_particle_y);
        let spin_button_hit = check_coords(WIDTH/2.0-292.0+57.0/2.0, HEIGHT/2.0-302.0+16.0, x, y, 57.0/2.0,14.0);
        if hit_particle {
            self.coords = Vec2::new(rand_coord_gen_low(X_MIN, X_MAX), Y_MAX);
            self.particle.color = rand_double();
            self.particle.size = rand_double();
            self.particle.opacity = rand_double();
            self.particles_found += 1;
        } else if spin_button_hit {
            println!("Spin button hit");
            self.hit_spin = true;
        }
        Ok(())
    }
}

fn rand_coord_gen(x_min:f32, x_max:f32, y_min:f32, y_max:f32) -> Vec2 {
    let mut rng = rand::thread_rng();
    let x_pos = rng.gen_range(x_min..x_max);
    let y_pos = rng.gen_range(y_min..y_max);
    Vec2::new(x_pos, y_pos)
}

fn rand_coord_gen_low(x_min:f32, x_max:f32) -> f32 {
    let mut rng = rand::thread_rng();
    let x_pos = rng.gen_range(x_min..x_max);
    return x_pos;
}

fn rand_double() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

fn check_coords(point_x: f32, point_y: f32, mouse_x: f32, mouse_y: f32, margin_x: f32, margin_y: f32) -> bool {
    let lower_x = point_x - margin_x;
    let upper_x = point_x + margin_x;
    let lower_y = point_y - margin_y;
    let upper_y = point_y + margin_y;
    return mouse_x > lower_x && mouse_x < upper_x && mouse_y > lower_y && mouse_y < upper_y;
}
pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("find_the_particle", "Joost_Hendriks")
    .add_resource_path(resource_dir)
    .window_mode(ggez::conf::WindowMode::default().dimensions(WIDTH, HEIGHT))
    .window_setup(WindowSetup::default()
    .title("Find the particle"))
    .build()
    .unwrap();

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state);
}
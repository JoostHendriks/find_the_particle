
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

struct MainState {
    dt: std::time::Duration,
    fps: f64,
    image1: graphics::Image,
    coords: Vec2,
    mouse_down: bool,
    particles_found: i32,
}

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image1 = graphics::Image::from_path(ctx, "/syringe.png")?;

        let s = MainState { 
            dt: std::time::Duration::new(0, 0), 
            fps: 0.0,
            image1,
            coords: coord_gen(355.0, 445.0, 270.0, 585.0),
            mouse_down: false,
            particles_found: 0,
            };
        Ok(s)
    }
}

impl ggez::event::EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        self.fps = ctx.time.fps();
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
            graphics::Rect::new(200.0, 40.0, 400.0, 700.0),
            25.0,
            Color::WHITE,
        )?;
        canvas.draw(&rectangle, Vec2::new(0.0,0.0));

        canvas.draw(
            graphics::Text::new(format!("Frame rate = {}ms {}Hz", self.dt.as_millis(), self.fps.round()))
                .set_scale(12.),
                Vec2::new(600.0,780.0),
        );

        let dst = ggez::glam::Vec2::new(220.0, 50.0);
        canvas.draw(&self.image1, graphics::DrawParam::new().dest(dst));

        let particle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            2.0,
            1.0,
            Color::BLACK,
        )?;
        canvas.draw(&particle, self.coords);

        // Draw score
        canvas.draw(
            graphics::Text::new(format!("Particles found {}", self.particles_found))
                .set_scale(24.),
                Vec2::new(5.0, 5.0),
        );
        let mut text: &str = " ";
        match self.particles_found {
            1 => text = "Good busy!",
            5 => text = "5 particles found already!",
            10 => text = "10 found! I would ask Joren for a raise",
            _ => (),
        }
        canvas.draw( graphics::Text::new(format!("{}", text))
            .set_scale(24.),
            Vec2::new(200.0, 750.0),
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
        let margin = 5.0;
        let hit = check_coords(self.coords.x, self.coords.y, x, y, margin);
        if hit {
            self.coords = coord_gen(355.0, 445.0, 270.0, 585.0);
            self.particles_found += 1;
        }
        Ok(())
    }
}

fn coord_gen(x_min:f32, x_max:f32, y_min:f32, y_max:f32) -> Vec2 {
    let mut rng = rand::thread_rng();
    let x_pos = rng.gen_range(x_min..x_max);
    let y_pos = rng.gen_range(y_min..y_max);
    Vec2::new(x_pos,y_pos)
}

fn check_coords(point_x: f32, point_y: f32, mouse_x: f32, mouse_y: f32, margin: f32) -> bool {
    let lower_x = point_x - margin;
    let upper_x = point_x + margin;
    let lower_y = point_y - margin;
    let upper_y = point_y + margin;
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
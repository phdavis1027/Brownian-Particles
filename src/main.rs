use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{ self, Color, Mesh, DrawMode };
use ggez::event::{self, EventHandler, MouseButton};
use glam::*;
use ggez::conf::{self, WindowMode};

fn main() -> GameResult {
    let (HEIGHT, WIDTH) = (600.0, 600.0);
    let cb = ggez::ContextBuilder::new("Particles", "Phillip Davis")
        .window_mode(
            conf::WindowMode::default()
                .fullscreen_type(conf::FullscreenType::Windowed)
                .resizable(true)
    );
    let (ctx, event_loop) = cb.build()?;

    let state = State::new(
        HEIGHT / 2.0,
        WIDTH / 2.0
    );
    event::run(ctx, event_loop, state);
}

struct State {
    pgen: ParticleGenerator,
    mouse_down: bool
}

impl State {
    fn new (x: f32, y: f32) -> Self {
        State {
            pgen: ParticleGenerator::new(
                x,
                y,
            ),
            mouse_down: false,
        }
    }
}

impl State{ 

    fn compute_logical_coordinates (&self, ctx: &mut Context, x: f32, y: f32) 
        -> GameResult<(f32, f32)>
    {
        let screen_rect = graphics::screen_coordinates(ctx);
        let size = graphics::window(ctx).inner_size();
        let loc_x = (x / (size.width  as f32)) * screen_rect.w + screen_rect.x;
        let loc_y = (y / (size.height as f32)) * screen_rect.h + screen_rect.y;
        Ok((loc_x, loc_y))
    }

    fn move_generator(&mut self, ctx: &mut Context, x: f32, y: f32) -> GameResult {
        let (x, y) = self.compute_logical_coordinates(ctx, x, y).unwrap();
        self.pgen.loc_x = x;
        self.pgen.loc_y = y;
        Ok(())
    }
}

struct ParticleGenerator {
    loc_x: f32,
    loc_y: f32,
    pbuf: [Particle; 10]
}

impl ParticleGenerator 
{
    fn new ( loc_x: f32, loc_y: f32) -> Self
    {
        ParticleGenerator {
           loc_x,
           loc_y,
           pbuf: [Particle {
               loc_x,
               loc_y,
               dir: 0.0,
           }; 10]
        }
    }
}

#[derive(Copy, Clone)]
struct Particle {
    loc_x: f32,
    loc_y: f32,
    dir: f32, // an angle
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        if self.mouse_down {
            self.move_generator(_ctx, x, y);
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        println!("x: {}, y: {}", x, y);
        self.move_generator(_ctx, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        let pgen_circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Vec2::new(
                self.pgen.loc_x,
                self.pgen.loc_y
            ),
            25.0,
            0.1,
            Color::BLACK
        )?;

        graphics::draw( // render the pgen center
            ctx,
            &pgen_circle,
            graphics::DrawParam::default()
        )?;
        graphics::present(ctx)
    }
}

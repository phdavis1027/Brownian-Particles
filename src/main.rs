use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{ self, Color, Mesh, DrawMode };
use ggez::event::{self, EventHandler, MouseButton};
use glam::*;
use ggez::conf::{self, WindowMode};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time;

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
    mouse_down: bool,
    delta: f32
}

impl State {
    fn new (x: f32, y: f32) -> Self {
        State {
            pgen: ParticleGenerator::new(
                x,
                y,
            ),
            mouse_down: false,
            delta: 0.1
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
    num_particles: usize,
    alive_particles: Vec<Particle>,
    survivors: Vec<Particle>,
    loc_x: f32,
    loc_y: f32,
    pbuf: [Particle; 10],
    rng: ThreadRng
}

#[derive(Copy, Clone, PartialEq)]
enum ParticleStatus {
    ALIVE,
    DEAD
}

impl ParticleGenerator 
{
    fn new( loc_x: f32, loc_y: f32) -> Self
    {
        ParticleGenerator {
           alive_particles: Vec::new(),
           survivors: Vec::new(), 
           num_particles: 0,
           loc_x,
           loc_y,
           pbuf: [Particle {
               loc_x,
               loc_y,
               dir: 0.0,
               death_date: time::Instant::now(),
               status: ParticleStatus::ALIVE
           }; 10],
           rng: ThreadRng::default()
        }
    }

    fn add_particle(&mut self) -> GameResult
    {
        if self.num_particles == self.pbuf.len() {
            self.flush_particles();
            self.num_particles = 0;
        }

        self.pbuf[self.num_particles] = Particle::new(
            self.loc_x,
            self.loc_y,
            &mut self.rng,
            time::Instant::now()
        ).unwrap();

        self.num_particles += 1;
        Ok(())
    }

    fn flush_particles(&mut self) -> GameResult
    {
        for particle in self.pbuf {
            self.alive_particles.push(particle);
        }
        self.num_particles = 0;
        Ok(())
    }

}

#[derive(Copy, Clone)]
struct Particle {
    loc_x: f32,
    loc_y: f32,
    dir: f32, // an angle
    death_date: time::Instant, 
    status: ParticleStatus
}

impl Particle {

    fn new(loc_x: f32, loc_y: f32, rng: &mut ThreadRng, right_now: time::Instant ) -> GameResult<Particle>
    {
        let rand_dir = rng.gen::<f32>() * 360.0;

        Ok(
            Particle {
                loc_x,
                loc_y,
                dir: rand_dir,
                death_date: right_now + time::Duration::from_secs(5),
                status: ParticleStatus::ALIVE 
            }
        )
    }

}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pgen.add_particle();
        self.pgen.alive_particles.retain(|p|{
            p.status == ParticleStatus::ALIVE
        });
        Ok(())
    }
        
        
    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        if self.mouse_down {
            self.move_generator(_ctx, x, y);
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = true;
        self.move_generator(_ctx, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_down = false;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);

        let right_now = time::Instant::now();

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

        /*
        graphics::draw( // render the pgen center
            ctx,
            &pgen_circle,
            graphics::DrawParam::default()
        )?;
        */
        for particle in &mut self.pgen.alive_particles{
            match particle.status {
                ParticleStatus::ALIVE => {
                    let particle_circle = Mesh::new_circle(
                        ctx,
                        DrawMode::fill(),
                        Vec2::new(
                            particle.loc_x,
                            particle.loc_y
                        ),
                        5.0,
                        0.1,
                        Color::BLACK
                    ).unwrap();

                    graphics::draw(
                        ctx,
                        &particle_circle,
                        graphics::DrawParam::default()
                    ).unwrap();

                    if particle.death_date >= right_now {
                        particle.status = ParticleStatus::DEAD;
                    }
                },
                ParticleStatus::DEAD =>{}
            }
        }
        graphics::present(ctx)
    }
}

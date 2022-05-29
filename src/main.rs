use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{ self, Color, Mesh, DrawMode };
use ggez::event::{self, EventHandler, MouseButton};
use glam::*;
use ggez::conf::{self, WindowMode};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time;
use dubble::DoubleBuffered;

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
    particles: DoubleBuffered<Vec<Particle>>,
    loc_x: f32,
    loc_y: f32,
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
           particles: DoubleBuffered::construct_with(
               Vec::<Particle>::new
           ),
           loc_x,
           loc_y,
           rng: ThreadRng::default()
        }
    }

    fn add_particle(&mut self) -> GameResult
    {
        let mut wb = self.particles.write();
        wb.push(
            Particle::new(
                self.loc_x,
                self.loc_y,
                &mut self.rng,
                time::Instant::now()
            ).unwrap()
        );
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

impl Default for ParticleStatus {
    fn default () -> Self {
        ParticleStatus::ALIVE
    }
}

impl Particle {

    fn update(&mut self) -> GameResult {
        self.loc_x += self.loc_x.cos() * 5.0;
        self.loc_y += self.loc_y.sin() * 5.0;
        Ok(())
    }

    fn new(loc_x: f32, loc_y: f32, rng: &mut ThreadRng, right_now: time::Instant ) -> GameResult<Particle>
    {
        let rand_dir = rng.gen::<f32>() * 360.0;
        Ok(
            Particle {
                loc_x,
                loc_y,
                dir: rand_dir,
                death_date: right_now + time::Duration::from_secs(10),
                status: ParticleStatus::ALIVE 
            }
        )
    }
}


impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pgen.add_particle();
        let mut wb = self.pgen.particles.write(); 
        wb.retain(|p|{
            p.status == ParticleStatus::ALIVE
        });
        self.pgen.particles.update();
        let mut wb = self.pgen.particles.write(); 
        wb.into_iter().for_each(|p|{
            p.update();
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
        
        let mut wb = self.pgen.particles.write();
        wb.into_iter().for_each(|p|{
            match p.status{
                ParticleStatus::ALIVE => {
                    let particle_circle = Mesh::new_circle(
                        ctx,
                        DrawMode::fill(),
                        Vec2::new(
                            p.loc_x,
                            p.loc_y
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

                    if p.death_date <= right_now {
                        p.status = ParticleStatus::DEAD;
                    }
                },
                ParticleStatus::DEAD =>{ }
            }
        });
        self.pgen.particles.update();
        graphics::present(ctx)
    }
}


impl Default for Particle {
    fn default () -> Self{
        Particle {
            loc_x: 0.0,
            loc_y: 0.0,
            dir: 0.0,
            death_date: time::Instant::now(), 
            status: ParticleStatus::ALIVE 
        }
    }
}
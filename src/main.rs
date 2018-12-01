//! A collection of semi-random shape and image drawing examples.

extern crate ggez;
extern crate rand;

#[macro_use]
extern crate typed_builder;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use std::env;
use std::path;

#[derive(TypedBuilder)]
struct MainState {
    point_a: Point,
    point_b: Point,
    point_c: Point,
    point_d: Point,
    point_e: Point,
    random: RandomPoint,
}

struct Point {
    x: f32,
    y: f32,
    point: Point2,
}

struct RandomPoint {
    x: f32,
    y: f32,
    point: Point2,
    vertex: usize,
}

impl RandomPoint {
    fn new(x: f32, y: f32, vertex: usize) -> Self {
        RandomPoint {
            x,
            y,
            point: Point2::new(x, y),
            vertex,
        }
    }
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Point {
            x,
            y,
            point: Point2::new(x, y),
        }
    }
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState::builder()
           .point_a(Point::new(800.0, 0.0))
           .point_b(Point::new(39.0, 553.0))
           .point_c(Point::new(330.0, 1447.0))
           .point_d(Point::new(1270.0, 1447.0))
           .point_e(Point::new(1561.0, 553.0))
           .random(RandomPoint::new(800.0, 0.0, 0))
           .build())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = rand::thread_rng();
        let mut newPoint = rng.gen_range(0, 5);
        let mut diff = (newPoint as i8 - self.random.vertex as i8).abs();
        while diff == 1 || diff == 4 {
            newPoint = rng.gen_range(0, 5);
            diff = (newPoint as i8 - self.random.vertex as i8).abs();
        }
        match newPoint {
            0 => {
                let midX = (self.point_a.x + self.random.x) / 2.0;
                let midY = (self.point_a.y + self.random.y) / 2.0;
                self.random = RandomPoint::new(midX, midY, 0);
                graphics::set_color(ctx, graphics::Color::from_rgb(0, 255, 0))?;
            }
            1 => {
                let midX = (self.point_b.x + self.random.x) / 2.0;
                let midY = (self.point_b.y + self.random.y) / 2.0;
                self.random = RandomPoint::new(midX, midY, 1);
                graphics::set_color(ctx, graphics::Color::from_rgb(0, 0, 255))?;
            }
            2 => {
                let midX = (self.point_c.x + self.random.x) / 2.0;
                let midY = (self.point_c.y + self.random.y) / 2.0;
                self.random = RandomPoint::new(midX, midY, 2);
                graphics::set_color(ctx, graphics::Color::from_rgb(255, 0, 0))?;
            }
            3 => {
                let midX = (self.point_d.x + self.random.x) / 2.0;
                let midY = (self.point_d.y + self.random.y) / 2.0;
                self.random = RandomPoint::new(midX, midY, 3);
                graphics::set_color(ctx, graphics::Color::from_rgb(255, 255, 0))?;
            }
            4 => {
                let midX = (self.point_e.x + self.random.x) / 2.0;
                let midY = (self.point_e.y + self.random.y) / 2.0;
                self.random = RandomPoint::new(midX, midY, 4);
                graphics::set_color(ctx, graphics::Color::from_rgb(0, 255, 255))?;
            }
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::circle(ctx, DrawMode::Fill, self.random.point, 1.0, 0.1)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let ctx = &mut ContextBuilder::new("text_cached", "ggez")
        .window_setup(WindowSetup::default().title("chaos").resizable(false))
        .window_mode(WindowMode::default().dimensions(1600, 1600).vsync(false))
        .build()
        .unwrap();

    // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    println!("{}", graphics::get_renderer_info(ctx).unwrap());
    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

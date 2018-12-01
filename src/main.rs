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
    random: Point,
}

struct Point {
    x: f32,
    y: f32,
    point: Point2,
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
            .point_a(Point::new(10.0, 1590.0))
            .point_b(Point::new(1590.0, 1590.0))
            .point_c(Point::new(790.0, 200.0))
            .random(Point::new(10.0, 1590.0))
            .build())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 3) {
            0 => {
                let midX = (self.point_a.x + self.random.x) / 2.0;
                let midY = (self.point_a.y + self.random.y) / 2.0;
                self.random = Point::new(midX, midY);
                graphics::set_color(ctx, graphics::Color::from_rgb(0, 255, 0))?;
            }
            1 => {
                let midX = (self.point_b.x + self.random.x) / 2.0;
                let midY = (self.point_b.y + self.random.y) / 2.0;
                self.random = Point::new(midX, midY);
                graphics::set_color(ctx, graphics::Color::from_rgb(0, 0, 255))?;
            }
            2 => {
                let midX = (self.point_c.x + self.random.x) / 2.0;
                let midY = (self.point_c.y + self.random.y) / 2.0;
                self.random = Point::new(midX, midY);
                graphics::set_color(ctx, graphics::Color::from_rgb(255, 0, 0))?;
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

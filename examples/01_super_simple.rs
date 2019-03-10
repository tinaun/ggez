//! The simplest possible example that does something.

extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 100.0),
            100.0,
            0.5,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        let bircle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 100.0),
            100.0,
            0.5,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &bircle, (na::Point2::new(0.0, 200.0),))?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 400.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn resize_event(&mut self, ctx: &mut Context, w: f32, h: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0., 0., w, h)).unwrap();
        //graphics::set_resolution(ctx, w, h).unwrap();
        println!("{:?}", graphics::drawable_size(ctx));
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_mode(
        ggez::conf::WindowMode::default().resizable(true)
    );
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}

use ggez::{Context, GameResult, glam::Vec2, graphics::{Canvas, Color, DrawMode, DrawParam, Mesh}, mint::{Point2, Vector2}};

use crate::math;

pub struct Car {
    pub pos: Point2<f32>,
    pub dir: Vector2<f32>,
    pub speed: u16,
    pub dest: Point2<f32>
}

impl Car {
    pub const fn new(pos: Point2<f32>, dir: Vector2<f32>, speed: u16) -> Self {
        Car {
            pos: pos,
            dir: dir,
            speed: speed,
            dest: Point2 { x: pos.x, y: pos.y }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            self.pos,
            10.0,
            1.0,
            Color::RED,
        )?;

        let draw_param = DrawParam::default();

        canvas.draw(&circle, draw_param);

        Ok(())
    }

    pub fn point_to(&mut self, destination: &Vec2) {
        let dir = math::direction(&self.pos.into(), destination);
        
        self.dir.x = dir.x;
        self.dir.y = dir.y;
    }
}

use ggez::{Context, GameResult, glam::Vec2, graphics::Canvas, mint::Point2};

use crate::{entity::car::Car, tile};

pub mod car;
pub struct EntityManager<const N: usize>{
    pub car_pool: [Car; N],
}

impl<const N: usize> EntityManager<N> {
    pub fn update_cars(&mut self, _ctx: &mut Context) -> GameResult {
        for car in &mut self.car_pool {
            let destination: Vec2 = tile::tile_center(&Point2 { x: 100.0, y: 100.0 }, &Point2 { x: 0.0, y: 0.0 }).into();
            let distance = destination.distance_squared(Vec2::from((car.pos.x, car.pos.y)));

            if distance < car.speed as f32 {
                car.pos.x = destination.x;
                car.pos.y = destination.y;
                car.dir.x = 0.0;
                car.dir.y = 0.0;
            } else {
                car.point_to(&destination);

                car.pos.x += car.dir.x * car.speed as f32;
                car.pos.y += car.dir.y * car.speed as f32;
            }

            println!("Car: speed: {}, dir: x: {}, y: {}, distance: {}", car.speed, car.dir.x, car.dir.y, distance);
        }

        Ok(())
    }

    pub fn draw_cars(&self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        for car in &self.car_pool {
            car.draw(canvas, ctx)?;
        }

        Ok(())
    }
}
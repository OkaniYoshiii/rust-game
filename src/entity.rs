use ggez::{Context, GameResult, graphics::Canvas};

use crate::entity::car::Car;

pub mod car;
pub struct EntityManager<const N: usize>{
    pub car_pool: [Car; N],
}

impl<const N: usize> EntityManager<N> {
    pub fn draw_cars(&self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        for car in &self.car_pool {
            car.draw(canvas, ctx)?;
        }

        Ok(())
    }
}
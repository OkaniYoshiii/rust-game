use ggez::{glam::Vec2};

pub fn direction(position: &Vec2, destination: &Vec2) -> Vec2 {
    let dist_x = destination.x - position.x;
    let dist_y = destination.y - position.y;

    let distance = Vec2 {
        x: dist_x,
        y: dist_y,
    };

    distance.normalize_or_zero()
}
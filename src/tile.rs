use ggez::{Context, GameResult};
use ggez::{mint::Vector2};
use ggez::graphics::{self, Canvas, Color, Mesh, Rect};
use rand::Rng;

const TILE_WIDTH: i32 = 100;
const TILE_HEIGHT: i32 = 50;

/// For an isometric grid like this one:
///          /x:0,y:0/
///     /x:0,y:1/ /x:1,y:0/
/// /x:0,y:2/ /x:1,y:1/ /x:2,y:0/
///     /x:1,y:2/ /x:2,y:1/
///          /x:2,y:2/
/// 
/// Given a position on the grid, returns the screen
/// coordinates where to draw a tile
/// 
/// See: https://pikuma.com/blog/isometric-projection-in-games
/// 
/// origin: La position sur l'écran de la tile à la position: 0,0
/// tile_pos: la position x et y sur la grille de la tile à poser
pub fn tile_position(origin: Vector2<i32>, tile_pos: Vector2<i32>) -> Vector2<i32> {
    let screen_x = origin.x + ((tile_pos.x - tile_pos.y) * TILE_WIDTH / 2);
    let screen_y = origin.y + ((tile_pos.x + tile_pos.y) * TILE_HEIGHT / 2);

    Vector2{
        x: screen_x,
        y: screen_y,
    }
}

pub fn draw_tile(ctx: &mut Context, canvas: &mut Canvas, screen_pos: Vector2<i32>) -> GameResult {
    let mut rng = rand::rng();
    let color = Color::from_rgb(
        rng.random_range(0..255),
        rng.random_range(0..255),
        rng.random_range(0..255),
    );

    let rect = Rect::new(
        screen_pos.x as f32,
        screen_pos.y as f32,
        TILE_WIDTH as f32,
        TILE_HEIGHT as f32,
    );

    let rect_mesh = Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        rect,
        color,
    )?;

    canvas.draw(&rect_mesh, graphics::DrawParam::default());

    Ok(())
}

#[cfg(test)]
mod tests {
    use ggez::mint::Vector2;

    use crate::tile::{tile_position};

    struct TilePosTest {
        origin: Vector2<i32>,
        tile_pos: Vector2<i32>,

        expected: Vector2<i32>,
    }

    #[test]
    fn assert_correct() {
        let origin = Vector2 { x: 100, y: 100 };
        let tests: Vec<TilePosTest> = Vec::from([
            TilePosTest{
                origin: origin,
                tile_pos: Vector2 { x: 0, y: 0 },
                expected: Vector2 { x: origin.x, y: origin.y }
            },
        ]);

        for test in tests {
            let position = tile_position(test.origin, test.tile_pos);

            assert_eq!(position.x, test.expected.x);
            assert_eq!(position.y, test.expected.y);
        }
    }
}

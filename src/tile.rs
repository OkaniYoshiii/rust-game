use ggez::mint::Point2;
use ggez::{Context, GameResult};
use ggez::{mint::Vector2};
use ggez::graphics::{self, Canvas, DrawParam, Image, Transform};

const TILE_ASPECT_RATIO: f32 = 1.0 / 2.0;
const TILE_WIDTH: i32 = 100;
const TILE_HEIGHT: i32 = ((TILE_WIDTH as f32) * TILE_ASPECT_RATIO) as i32;

pub enum Tile {
    None,
    Home,
}

pub fn draw_tilemap(canvas: &mut Canvas, ctx: &mut Context, origin: &Point2<f32>, tilemap: &[&[Tile]]) -> GameResult {
    for (y, row) in tilemap.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let tile_pos = Point2{ x: x as i32, y: y as i32};
            let screen_position = tile_position(origin, &tile_pos);
            match tile {
                Tile::Home => draw_home_tile(canvas, ctx, screen_position)?,
                Tile::None => draw_empty_tile(canvas, ctx,screen_position)?,
            }
        }
    }

    Ok(())
}

pub fn draw_home_tile(canvas: &mut Canvas, ctx: &mut Context, screen_pos: Point2<f32>) -> GameResult {
    let image = graphics::Image::from_path(ctx, "/home.png")?;

    let dest = Point2 {
        x: screen_pos.x,
        y: screen_pos.y - 50.0,
    };

    draw_tile(canvas, &image, dest);

    Ok(())
}

pub fn draw_empty_tile(canvas: &mut Canvas, ctx: &mut Context, screen_pos: Point2<f32>) -> GameResult {
    let image = graphics::Image::from_path(ctx, "/empty.png")?;

    let dest = Point2 {
        x: screen_pos.x,
        y: screen_pos.y - 50.0,
    };

    draw_tile(canvas, &image, dest);

    Ok(())
}

pub fn draw_tile(canvas: &mut Canvas, image: &Image, dest: impl Into<Point2<f32>>) {
    let draw_param = DrawParam{
        transform: Transform::Values {
            dest: dest.into(),
            scale: Vector2 { x: 1.0, y: 1.0 },
            offset: Point2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
        },
        ..Default::default()
    };

    canvas.draw(image, draw_param);
}
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
pub fn tile_position(origin: &Point2<f32>, tile_pos: &Point2<i32>) -> Point2<f32> {
    let screen_x = origin.x + ((tile_pos.x - tile_pos.y) * (TILE_WIDTH / 2)) as f32;
    let screen_y = origin.y + ((tile_pos.x + tile_pos.y) * (TILE_HEIGHT / 2)) as f32;

    Point2 {
        x: screen_x as f32,
        y: screen_y as f32,
    }
}

// pub fn draw_tile(ctx: &mut Context, canvas: &mut Canvas, screen_pos: &Vector2<i32>) -> GameResult {
//     let mut rng = rand::rng();
//     let color = Color::from_rgb(
//         rng.random_range(0..255),
//         rng.random_range(0..255),
//         rng.random_range(0..255),
//     );

//     let rect = Rect::new(
//         screen_pos.x as f32,
//         screen_pos.y as f32,
//         TILE_WIDTH as f32,
//         TILE_HEIGHT as f32,
//     );

//     let rect_mesh = Mesh::new_rectangle(
//         ctx,
//         graphics::DrawMode::fill(),
//         rect,
//         color,
//     )?;

//     canvas.draw(&rect_mesh, graphics::DrawParam::default());

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use ggez::mint::{Point2};

    use crate::tile::{tile_position};

    struct TilePosTest {
        origin: Point2<f32>,
        tile_pos: Point2<i32>,

        expected: Point2<f32>,
    }

    #[test]
    fn assert_correct() {
        let origin = Point2 { x: 100.0, y: 100.0 };
        let tests: Vec<TilePosTest> = Vec::from([
            TilePosTest{
                origin: origin,
                tile_pos: Point2 { x: 0, y: 0 },
                expected: Point2 { x: origin.x, y: origin.y }
            },
        ]);

        for test in tests {
            let position = tile_position(&test.origin, &test.tile_pos);

            assert_eq!(position.x, test.expected.x);
            assert_eq!(position.y, test.expected.y);
        }
    }
}
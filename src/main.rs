use std::time::Duration;

use game::{entity::{EntityManager, car::Car}, tile::{self, Tile, Tilemap}};
use ggez::{self, Context, ContextBuilder, GameResult, conf::{Conf, WindowMode, WindowSetup}, event, graphics::{Canvas, Color, DrawParam, Text}, input::keyboard::{self, KeyCode}, mint::{Point2, Vector2}};

const TARGET_FPS: u32 = 30;

const TILEMAP: Tilemap<3, 3> = Tilemap::new(
    Point2 { x: 100.0, y: 100.0 },
    [
        [Tile::Home, Tile::Home, Tile::Home],
        [Tile::None, Tile::None, Tile::None],
        [Tile::Home, Tile::Home, Tile::Home],
    ],
);

/// # Game State
/// 
/// Represent the game state
/// 
/// Stores all data related to represent the game state such as
/// player position, scores, cards etc ...
struct State<const N: usize, const W: usize, const H: usize> where {
    tilemap: Tilemap<W, H>,
    delta_time: Duration,
    entity_manager: EntityManager<N>,
}

impl<const N: usize, const W: usize, const H: usize> ggez::event::EventHandler for State<N, W, H> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.delta_time = ctx.time.delta();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        
        self.tilemap.draw(&mut canvas, ctx)?;

        let text = Text::new("Rust game");
        let params = DrawParam{
            color: Color::BLACK,
            ..Default::default()
        };

        self.entity_manager.draw_cars(&mut canvas, ctx)?;

        canvas.draw(&text, params);
        canvas.finish(ctx)?;

        println!("Delta time : {}ns", self.delta_time.as_nanos());
        Ok(())
    }
}
fn main() {
    let window_setup = WindowSetup{
        title: "Rust Game".to_owned(),
        vsync: true,
        ..Default::default()
    };

    let window_mode = WindowMode {
        width: 960.0,
        height: 460.0,
        ..Default::default()
    };

    let car_pool: [Car; 1] = [
        Car::new(
            tile::tile_position(&TILEMAP.origin, &Point2 { x: 0.0, y: 0.0 }),
            Vector2 { x: 1.0, y: 0.0 },
            5
        ),
    ];

    let state = State{
        tilemap: TILEMAP,
        delta_time: Duration::new(0, 0),
        entity_manager: EntityManager {
            car_pool: car_pool,
        }
    };

    let conf = Conf {
        window_setup: window_setup,
        window_mode: window_mode,
        ..Default::default()
    };

    let (ctx, event_loop) = ContextBuilder::new("game", "OkaniYoshiii")
        .default_conf(conf)
        .add_resource_path("./resources")
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
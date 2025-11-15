use std::time::Duration;

use game::tile::{self, Tile};
use ggez::{self, Context, ContextBuilder, GameResult, conf::{Conf, WindowMode, WindowSetup}, event, graphics::{Canvas, Color, DrawParam, Text}, mint::Vector2};

const TARGET_FPS: u32 = 30;

const TILEMAP: [&'static[Tile]; 3] = [
    &[Tile::Home, Tile::Home, Tile::Home],
    &[Tile::None, Tile::None, Tile::None],
    &[Tile::Home, Tile::Home, Tile::Home],
];

/// # Game State
/// 
/// Represent the game state
/// 
/// Stores all data related to represent the game state such as
/// player position, scores, cards etc ...
struct State {
    delta_time: Duration,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.delta_time = ctx.time.delta();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        
        let origin = Vector2 { x: 100.0, y: 100.0 };

        tile::draw_tilemap(&mut canvas, ctx, &origin, &TILEMAP)?;

        let text = Text::new("Rust game");
        let params = DrawParam{
            color: Color::BLACK,
            ..Default::default()
        };

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

    let state = State{
        delta_time: Duration::new(0, 0),
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
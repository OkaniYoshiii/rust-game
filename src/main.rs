use std::time::Duration;

use game::tile::{self, Grid, tile_position};
use ggez::{self, Context, ContextBuilder, GameResult, conf::{Conf, WindowMode, WindowSetup}, event, graphics::{Canvas, Color, DrawParam, Text}, mint::Vector2};

const UPDATE_RATE: u32 = 30;

/// # Game State
/// 
/// Represent the game state
/// 
/// Stores all data related to represent the game state such as
/// player position, scores, cards etc ...
struct State {
    delta_time: Duration,
    grid: Grid,
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while !ctx.time.check_update_time(UPDATE_RATE) {
            return Ok(());
        }

        self.delta_time = ctx.time.delta();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        
        for (x, row) in self.grid.cells.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let position: Vector2<i32> = Vector2 { x: x as i32, y: y as i32 };
                let screen_pos = tile_position(&self.grid.origin, &position);
    
                tile::draw_tile(ctx, &mut canvas, &screen_pos, tile)?;
            }
        }

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
        grid: Grid::new(Vector2 { x: 300, y: 15 }),
    };

    let conf = Conf {
        window_setup: window_setup,
        window_mode: window_mode,
        ..Default::default()
    };

    let (ctx, event_loop) = ContextBuilder::new("game", "OkaniYoshiii")
        .default_conf(conf)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
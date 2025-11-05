use std::time::Duration;

use ggez::{self, Context, ContextBuilder, GameResult, conf::{Conf, WindowMode, WindowSetup}, event, graphics::{Canvas, Color, DrawParam, Text}};

const TARGET_FPS: u32 = 30;

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
        while !ctx.time.check_update_time(TARGET_FPS) { return Ok(()); }

        self.delta_time = ctx.time.delta();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        while !ctx.time.check_update_time(TARGET_FPS) { return Ok(()); }

        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
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
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
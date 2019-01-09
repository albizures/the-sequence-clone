extern crate ggez;

mod power;

use ggez::{
    graphics::{Image},
    conf,
    Context,
    GameResult,
    event,
};

use std::env;
use std::path;

use self::power::load_sprite;

struct MainState {
    power: Image,
}


impl MainState {
    fn new(context: &mut Context) -> GameResult<MainState> {
        let power = load_sprite(context)?;
        let state = MainState {
            power
        };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let config = conf::Conf::new();
    let context = &mut Context::load_from_conf("The Sequence", "Jose Albizures", config)?;

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        context.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(context)?;
    event::run(context, state)?;

    Ok(())
}
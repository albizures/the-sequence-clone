
use ggez::graphics::{Image, FilterMode};
use ggez::Context;
use ggez::GameResult;

pub fn load_sprite (context: &mut Context) -> GameResult<Image> {
  let mut sprite = Image::new(context, "/power.png")?;
  sprite.set_filter(FilterMode::Nearest);

  Ok(sprite)
}
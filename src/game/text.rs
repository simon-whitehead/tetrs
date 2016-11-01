use std::any::Any;

use piston_window::*;
use piston_window::character::CharacterCache;

pub fn render<S, C, G>(text: S,
                       font_size: u32,
                       x: usize,
                       y: usize,
                       color: [f32; 4],
                       cache: &mut C,
                       context: Context,
                       gfx: &mut G)
    where S: Into<String>,
          C: CharacterCache,
          G: Graphics<Texture = <C as CharacterCache>::Texture>
{
    let transform = context.transform.trans(x as f64, y as f64);

    Text::new_color(color, font_size)
        .draw(&text.into()[..], cache, &context.draw_state, transform, gfx);
}

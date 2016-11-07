use piston_window::{Context, Graphics, Texture, Transformed};
use piston_window::character::CharacterCache;

pub struct Text {
    text: String,
    font_size: u32,
    x: usize,
    y: usize,
    color: [f32; 4],
}

impl Text {
    pub fn new<S>(text: S, font_size: u32, x: usize, y: usize, color: [f32; 4]) -> Text
        where S: Into<String>
    {
        Text {
            text: text.into(),
            font_size: font_size,
            x: x,
            y: y,
            color: color,
        }
    }

    pub fn set_text<S>(&mut self, text: S)
        where S: Into<String>
    {
        self.text = text.into();
    }

    pub fn render<C, G>(&self, cache: &mut C, context: Context, gfx: &mut G)
        where C: CharacterCache,
              G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let transform = context.transform.trans(self.x as f64, self.y as f64);

        ::piston_window::Text::new_color(self.color, self.font_size)
            .draw(&self.text[..], cache, &context.draw_state, transform, gfx);
    }
}

use piston_window::{Context, Graphics, Texture, Transformed};
use piston_window::character::CharacterCache;

pub struct Score {
    score: u32,
    location: (f64, f64),
    color: [f32; 4],
    font_size: u32,
}

impl Score {
    pub fn new() -> Score {
        Score {
            score: 0,
            location: (320.0, 29.0),
            color: [0.0, 0.0, 0.0, 1.0],
            font_size: 16,
        }
    }

    pub fn add(&mut self, value: u32) {
        self.score = self.score + value;
    }

    pub fn render<C, G>(&self, cache: &mut C, context: Context, gfx: &mut G)
        where C: CharacterCache,
              G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let score_label_transform = context.transform
            .trans(self.location.0 as f64, self.location.1 as f64);

        let score_number_transform = context.transform
            .trans(self.location.0 as f64,
                   self.location.1 + (self.font_size + (self.font_size / 3)) as f64);

        let score = format!("{}", self.score);

        ::piston_window::Text::new_color(self.color, self.font_size - (self.font_size / 3))
            .draw("Score",
                  cache,
                  &context.draw_state,
                  score_label_transform,
                  gfx);

        ::piston_window::Text::new_color(self.color, self.font_size).draw(&score[..],
                                                                          cache,
                                                                          &context.draw_state,
                                                                          score_number_transform,
                                                                          gfx);
    }
}
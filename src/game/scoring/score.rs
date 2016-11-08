use piston_window::{Context, Graphics, Texture, Transformed};
use piston_window::character::CharacterCache;

use game::render_options::RenderOptions;

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

    pub fn render<'a, C, G>(&self, options: &mut RenderOptions<'a, G, C>)
        where C: CharacterCache,
              G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let score_label_transform = options.context
            .transform
            .trans(self.location.0 as f64, self.location.1 as f64);

        let score_number_transform = options.context
            .transform
            .trans(self.location.0 as f64,
                   self.location.1 + (self.font_size + (self.font_size / 3)) as f64);

        let score = format!("{}", self.score);

        ::piston_window::Text::new_color(self.color, self.font_size - (self.font_size / 3))
            .draw("Score",
                  options.character_cache,
                  &options.context.draw_state,
                  score_label_transform,
                  options.graphics);

        ::piston_window::Text::new_color(self.color, self.font_size).draw(&score[..],
                                                                          options.character_cache,
                                                                          &options.context
                                                                              .draw_state,
                                                                          score_number_transform,
                                                                          options.graphics);
    }
}
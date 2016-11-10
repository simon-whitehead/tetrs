use piston_window::*;
use piston_window::character::CharacterCache;

use game::config::Config;
use game::render_options::RenderOptions;

pub struct Level {
    pub level: u32,
    location: (f64, f64),
    color: [f32; 4],
    font_size: u32,
}

impl Level {
    pub fn new(config: &Config) -> Level {
        Level {
            level: 1,
            location: (320.0, 129.0),
            color: config.ui_color,
            font_size: 16,
        }
    }

    pub fn increase(&mut self) {
        self.level += 1;
    }

    pub fn render<'a, C, G>(&self, options: &mut RenderOptions<'a, G, C>)
        where C: CharacterCache,
              G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let level_label_transform = options.context
            .transform
            .trans(self.location.0 as f64, self.location.1 as f64);

        let level_number_transform = options.context
            .transform
            .trans(self.location.0 as f64,
                   self.location.1 + (self.font_size + (self.font_size / 3)) as f64);

        let level = format!("{}", self.level);

        ::piston_window::Text::new_color(self.color, self.font_size - (self.font_size / 3))
            .draw("Level",
                  options.character_cache,
                  &options.context.draw_state,
                  level_label_transform,
                  options.graphics);

        ::piston_window::Text::new_color(self.color, self.font_size).draw(&level[..],
                                                                          options.character_cache,
                                                                          &options.context
                                                                              .draw_state,
                                                                          level_number_transform,
                                                                          options.graphics);
    }
}
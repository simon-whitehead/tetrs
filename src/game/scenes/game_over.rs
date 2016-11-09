use gfx_device_gl::Factory;
use piston_window::*;

use game::asset_factory::AssetFactory;
use game::config::Config;
use game::render_options::RenderOptions;
use game::scenes::{Scene, SceneResult};
use game::window::GameWindow;

pub struct GameOver {
    config: Config,
    asset_factory: AssetFactory,

    game_over_label: ::game::text::Text,
}

impl Scene for GameOver {
    fn process(&mut self, e: &Event) -> SceneResult {
        SceneResult::None
    }

    fn render(&mut self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |mut c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            let mut options = RenderOptions {
                config: &self.config,
                context: &mut c,
                graphics: g,
                character_cache: self.asset_factory.font.as_mut().unwrap(),
            };

            self.game_over_label.render(&mut options);
        });
    }
}

impl GameOver {
    pub fn new(config: Config, gfx_factory: Factory) -> GameOver {
        GameOver {
            config: config,
            asset_factory: AssetFactory::new(gfx_factory),
            game_over_label: ::game::text::Text::new("Game Over", 36, 100, 264, config.ui_color),
        }
    }
}
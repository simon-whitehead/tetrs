
use gfx_device_gl::Factory;
use piston_window::*;

use game::asset_factory::AssetFactory;
use game::config::Config;
use game::scenes::{Scene, SceneResult};
use game::window::GameWindow;

pub struct MainMenu {
    asset_factory: AssetFactory,
    selected_item: SelectedMenuItem,

    // Menu items
    new_game_label: ::game::text::Text,
    quit_label: ::game::text::Text,
}

#[derive(Copy, Clone)]
enum SelectedMenuItem {
    NewGame,
    Quit,
}

impl Scene for MainMenu {
    fn process(&mut self, e: &Event) -> SceneResult {
        match *e {
            Event::Input(ref input_event) => {
                if let Input::Press(ref button) = *input_event {
                    match *button {
                        Button::Keyboard(Key::Up) => self.up(),
                        Button::Keyboard(Key::Down) => self.down(),
                        Button::Keyboard(Key::Return) => {
                            match self.selected_item {
                                SelectedMenuItem::NewGame => SceneResult::NewGame,
                                SelectedMenuItem::Quit => SceneResult::Quit,
                            }
                        }
                        _ => SceneResult::None,
                    }
                } else {
                    SceneResult::None
                }
            }
            _ => SceneResult::None,
        }
    }

    fn render(&mut self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |mut c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            self.new_game_label.render(self.asset_factory.font.as_mut().unwrap(), c, g);
            self.quit_label.render(self.asset_factory.font.as_mut().unwrap(), c, g);
        });
    }
}

impl MainMenu {
    pub fn new(config: &Config, gfx_factory: Factory) -> MainMenu {
        MainMenu {
            asset_factory: AssetFactory::new(gfx_factory),
            selected_item: SelectedMenuItem::NewGame,
            new_game_label: ::game::text::Text::new("New Game", 24, 50, 200, config.ui_color),
            quit_label: ::game::text::Text::new("Quit", 16, 50, 250, config.ui_color),
        }
    }

    /// Move the selection up
    fn up(&mut self) -> SceneResult {
        match self.selected_item {
            SelectedMenuItem::NewGame => self.selected_item = SelectedMenuItem::Quit,
            SelectedMenuItem::Quit => self.selected_item = SelectedMenuItem::NewGame,
        }

        self.handle_selected_item();

        SceneResult::None
    }

    /// Move the selection down
    fn down(&mut self) -> SceneResult {
        match self.selected_item {
            SelectedMenuItem::NewGame => self.selected_item = SelectedMenuItem::Quit,
            SelectedMenuItem::Quit => self.selected_item = SelectedMenuItem::NewGame,
        }

        self.handle_selected_item();

        SceneResult::None
    }

    fn handle_selected_item(&mut self) {
        match self.selected_item {
            SelectedMenuItem::NewGame => {
                self.new_game_label.set_font_size(24);
                self.quit_label.set_font_size(16);
            }
            SelectedMenuItem::Quit => {
                self.new_game_label.set_font_size(16);
                self.quit_label.set_font_size(24);
            }
        }
    }
}
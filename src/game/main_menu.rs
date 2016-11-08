
use gfx_device_gl::Factory;
use piston_window::*;

use game::asset_factory::AssetFactory;
use game::config::Config;
use game::scene::{Scene, SceneResult};
use game::window::GameWindow;

pub struct MainMenu {
    asset_factory: AssetFactory,
    selected_item: SelectedMenuItem,
    selection: Option<SelectedMenuItem>,

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
                        Button::Keyboard(Key::Return) => self.make_selection(),
                        _ => (),
                    }

                    self.handle_selected_item();
                }
            }
            _ => (),
        }

        if let Some(ref selected_item) = self.selection {
            match *selected_item {
                SelectedMenuItem::NewGame => SceneResult::NewGame,
                SelectedMenuItem::Quit => SceneResult::Quit,
            }
        } else {
            SceneResult::None
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
            selection: None,
            new_game_label: ::game::text::Text::new("New Game", 24, 50, 200, config.ui_color),
            quit_label: ::game::text::Text::new("Quit", 16, 50, 250, config.ui_color),
        }
    }

    /// Move the selection up
    fn up(&mut self) {
        match self.selected_item {
            SelectedMenuItem::NewGame => self.selected_item = SelectedMenuItem::Quit,
            SelectedMenuItem::Quit => self.selected_item = SelectedMenuItem::NewGame,
        }
    }

    /// Move the selection down
    fn down(&mut self) {
        match self.selected_item {
            SelectedMenuItem::NewGame => self.selected_item = SelectedMenuItem::Quit,
            SelectedMenuItem::Quit => self.selected_item = SelectedMenuItem::NewGame,
        }
    }

    /// Make the final selection
    fn make_selection(&mut self) {
        self.selection = Some(self.selected_item.clone());
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
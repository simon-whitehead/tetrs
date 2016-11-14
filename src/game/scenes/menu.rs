use gfx_device_gl::Factory;
use piston_window::*;

use game::asset_factory::AssetFactory;
use game::config::Config;
use game::render_options::RenderOptions;
use game::scenes::{Scene, SceneResult};
use game::window::GameWindow;

pub struct Menu {
    config: Config,
    asset_factory: AssetFactory,
    menu_items: Vec<MenuItem>,
    selected_index: usize,
}

impl Menu {
    pub fn new(config: Config, gfx_factory: Factory) -> Menu {
        Menu {
            config: config,
            asset_factory: AssetFactory::new(gfx_factory),
            menu_items: Vec::new(),
            selected_index: 0,
        }
    }

    fn up(&mut self) -> MenuResult {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }

        MenuResult::None
    }

    fn down(&mut self) -> MenuResult {
        if self.selected_index < self.menu_items.len() - 1 {
            self.selected_index += 1;
        }

        MenuResult::None
    }

    pub fn add_item<S>(&mut self, text: S, result: MenuResult)
        where S: Into<String>
    {
        let current_length = self.menu_items.len();
        self.menu_items.push(MenuItem::new(text, result, current_length));
    }
}

impl Scene for Menu {
    fn process(&mut self, e: &Event) -> SceneResult {
        match *e {
            Event::Input(ref input_event) => {
                if let Input::Press(ref button) = *input_event {
                    SceneResult::from(match *button {
                        Button::Keyboard(Key::Up) => self.up(),
                        Button::Keyboard(Key::Down) => self.down(),
                        Button::Keyboard(Key::Return) => {
                            self.menu_items[self.selected_index].result
                        }
                        _ => MenuResult::None,
                    })
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

            let mut options = RenderOptions {
                config: &self.config,
                context: &mut c,
                graphics: g,
                character_cache: self.asset_factory.font.as_mut().unwrap(),
            };

            for (index, item) in self.menu_items.iter_mut().enumerate() {
                if self.selected_index == index {
                    item.label.set_font_size(24);
                } else {
                    item.label.set_font_size(16);
                }
                item.label.render(&mut options);
            }
        });
    }
}

pub struct MenuItem {
    result: MenuResult,
    label: ::game::text::Text,
}

impl MenuItem {
    fn new<S>(text: S, result: MenuResult, item_count: usize) -> MenuItem
        where S: Into<String>
    {
        MenuItem {
            result: result,
            label: ::game::text::Text::new(text.into(), 16, 50, 200 + (item_count * 50), [1.0; 4]),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MenuResult {
    None,
    NewGame,
    Quit,
}
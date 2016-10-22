use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

use piston_window::*;

use ::game::Game;

pub struct GameWindow {
    width: u32,
    height: u32,
    piston_window: PistonWindow,
    game: Game,
}

impl GameWindow {
    pub fn new<S>(width: u32, height: u32, title: S) -> GameWindow
        where S: Into<String>
    {
        GameWindow {
            piston_window: Self::create_window(width, height, title.into()),
            game: Game::new(),
            width: width,
            height: height,
        }
    }

    fn create_window(width: u32, height: u32, title: String) -> PistonWindow {
        WindowSettings::new(title, (width, height))
            .exit_on_esc(true)
            .opengl(OpenGL::V3_2)
            .build()
            .unwrap()
    }

    pub fn process(&mut self, e: &Event) -> bool {
        self.game.process(&e)
    }

    pub fn render(&self) {}
}

impl Deref for GameWindow {
    type Target = PistonWindow;

    fn deref(&self) -> &PistonWindow {
        &self.piston_window
    }
}

impl DerefMut for GameWindow {
    fn deref_mut(&mut self) -> &mut PistonWindow {
        &mut self.piston_window
    }
}

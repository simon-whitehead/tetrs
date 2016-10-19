use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

use piston_window::*;

use game::Game;

pub struct GameWindow {
    width: usize,
    height: usize,
    piston_window: PistonWindow,
    game: Game,
}

impl GameWindow {
    pub fn new(width: u32, height: u32) -> GameWindow {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow = WindowSettings::new("piston: sprite", (width, height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

        let game = Game::new();

        GameWindow {
            piston_window: window,
            game: game,
            width: 640,
            height: 480,
        }
    }

    pub fn process(&self, e: &Event) -> bool {
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

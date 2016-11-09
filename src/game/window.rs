use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

use piston_window::*;

pub struct GameWindow {
    width: u32,
    height: u32,
    pub piston_window: PistonWindow,
}

impl GameWindow {
    pub fn new<S>(width: u32, height: u32, title: S) -> GameWindow
        where S: Into<String>
    {
        GameWindow {
            piston_window: Self::create_window(width, height, title.into()),
            width: width,
            height: height,
        }
    }

    fn create_window(width: u32, height: u32, title: String) -> PistonWindow {
        WindowSettings::new(title, (width, height))
            .opengl(OpenGL::V3_2)
            .build()
            .unwrap()
    }
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

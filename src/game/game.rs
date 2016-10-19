
use piston_window::*;

use ::game::window::GameWindow;

pub struct Game {
}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn process(&self, e: &Event) -> bool {
        true
    }

    pub fn render(&self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform,
                      g);
        });
    }
}


use piston_window::*;

use ::game::window::GameWindow;

pub struct Game {
    time: f64,
    rec_y: f64,
    next_block_move: f64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            time: 0.0,
            rec_y: 0.0,
            next_block_move: 1.0,
        }
    }

    pub fn process(&mut self, e: &Event) -> bool {
        match *e {
            Event::Update(update) => {
                if self.time - self.next_block_move > 0.5 {
                    self.rec_y = self.rec_y + 50.0;
                    self.next_block_move = self.time + 1.0;
                }
                self.time = self.time + update.dt;
            }
            _ => (),
        }
        true
    }

    pub fn render(&self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, self.rec_y, 100.0, 100.0], // rectangle
                      c.transform,
                      g);
        });
    }
}

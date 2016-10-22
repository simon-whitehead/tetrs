
use piston_window::*;

use ::game::window::GameWindow;

pub struct Game {
    time: f64,
    rec_y: f64,
    next_block_move: f64,
    next_block_move_time: f64,
}

impl Game {
    pub fn new() -> Game {
        Game {
            time: 0.0,
            rec_y: 0.0,
            next_block_move: 1.0,
            next_block_move_time: 0.5,
        }
    }

    pub fn process(&mut self, e: &Event) -> bool {
        match *e {
            Event::Update(update) => {
                self.time = self.time + update.dt;
            }
            _ => (),
        }

        self.drop_active_block();

        true
    }

    fn drop_active_block(&mut self) {
        // Move the active block down if its been longer than the wait time
        if self.time - self.next_block_move > self.next_block_move_time {
            self.rec_y = self.rec_y + 50.0;
            self.next_block_move = self.time + 1.0;
        }
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

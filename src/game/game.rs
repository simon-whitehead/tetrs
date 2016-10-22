use std::cell::Cell;
use std::rc::Rc;

use piston_window::*;

use ::engine::{GameWindow, Timer, TimerTickResult};

pub struct Game {
    time: Rc<Cell<f64>>,
    rec_y: f64,
    next_block_move: f64,
    next_block_move_time: f64,
    block_drop_timer: Timer,
}

impl Game {
    pub fn new() -> Game {
        let time = RcCell!(0.0);

        Game {
            time: time.clone(),
            rec_y: 20.0,
            next_block_move: 1.0,
            next_block_move_time: 0.5,
            block_drop_timer: Timer::new(0.5, time.clone()),
        }
    }

    pub fn process(&mut self, e: &Event) -> bool {
        match *e {
            Event::Update(update) => {
                // Update the globa game time
                self.update_time(update.dt);

                // Drop the current block if it needs dropping
                self.drop_current_block();
            }
            Event::Input(ref input_event) => {
                self.handle_input(input_event);
            }
            _ => (),
        }

        true
    }

    fn update_time(&mut self, delta: f64) {
        self.time.set(self.time.get() + delta);
    }

    fn drop_current_block(&mut self) {
        if self.block_drop_timer.elapsed() {
            self.rec_y = self.rec_y + 10.0;
            self.block_drop_timer.reset();
        }
    }

    fn handle_input(&mut self, input: &Input) {
        if let Input::Press(ref button) = *input {
            match *button {
                Button::Keyboard(Key::Space) => println!("Space pressed!"),
                Button::Keyboard(Key::Left) => println!("Left pressed!"),
                Button::Keyboard(Key::Right) => println!("Right pressed!"),
                Button::Keyboard(Key::Down) => println!("Down pressed!"),
                _ => (),
            }
        }
    }

    fn drop_active_block(&mut self) {
        // Move the active block down if its been longer than the wait time
        self.rec_y = self.rec_y + 50.0;
    }

    pub fn render(&self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [20.0, self.rec_y, 20.0, 20.0], // rectangle
                      c.transform,
                      g);
        });
    }
}

use std::cell::Cell;
use std::rc::Rc;

use piston_window::*;

use ::game::config::{Config, ConfigBuilder};
use ::game::grid::Grid;
use ::game::tetromino::{Tetromino, TetrominoFactory};
use ::game::timer::{Timer, TimerTickResult};
use ::game::window::GameWindow;

pub struct Game {
    time: Rc<Cell<f64>>,
    config: Config,
    grid: Grid,
    block_drop_timer: Timer,
    tetromino: Tetromino,
    tetromino_factory: TetrominoFactory,
}

impl Game {
    pub fn new() -> Game {
        let time = RcCell!(0.0);
        let factory = TetrominoFactory::new();
        let config = ConfigBuilder::new()
            .grid_offset(10.0)
            .tile_size(29.0)
            .build();

        Game {
            time: time.clone(),
            config: config,
            grid: Grid::new(),
            block_drop_timer: Timer::new(0.5, time.clone()),
            tetromino: factory.create(),
            tetromino_factory: factory,
        }
    }

    pub fn process(&mut self, e: &Event) -> bool {
        match *e {
            Event::Update(update) => {
                // Update the globa game time
                self.update_time(update.dt);

                // Drop the current block if it needs dropping
                self.drop_current_block();

                // Apply the currently active tetromino into the grid
                self.grid.apply_tetromino(&self.tetromino);
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
            self.tetromino.drop_down();
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

    pub fn render(&self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            self.grid.render(&self.config, c, g, &e);
        });
    }
}

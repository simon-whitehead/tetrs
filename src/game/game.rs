use std::cell::Cell;
use std::rc::Rc;

use piston_window::*;

use game::config::{Config, ConfigBuilder};
use game::factory::TetrominoFactory;
use game::grid::Grid;
use game::tetromino::{Direction, MoveResult, Rotation, RotationResult, Tetromino};
use game::timer::{Timer, TimerTickResult};
use game::window::GameWindow;

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
            .grid_size((10, 22))
            .grid_offset(10.0)
            .tile_size(29.0)
            .build();

        let tetromino = factory.create(&config);

        Game {
            time: time.clone(),
            config: config,
            grid: Grid::new(),
            block_drop_timer: Timer::new(0.5, time.clone()),
            tetromino: tetromino,
            tetromino_factory: factory,
        }
    }

    pub fn process(&mut self, e: &Event) -> bool {
        match *e {
            Event::Update(update) => {
                // Update the globa game time
                self.update_time(update.dt);

                // Drop the current block if it needs dropping
                self.drop_current_block(None);

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

    /// Increments the global time
    fn update_time(&mut self, delta: f64) {
        self.time.set(self.time.get() + delta);
    }

    fn drop_current_block<O>(&mut self, force_drop: O)
        where O: Into<Option<bool>>
    {
        // If we can drop, check if we are ready and drop the active tetromino
        match self.tetromino.can_move(Direction::South, &self.grid.boxes) {
            MoveResult::Allow => {
                if self.block_drop_timer.elapsed() || force_drop.into().is_some() {
                    self.tetromino.drop_down();
                    self.block_drop_timer.reset();
                }
            }
            MoveResult::Blocked => {
                // Otherwise, store the tetromino in the grid and create a new tetromino
                self.new_tetromino();
            }
            _ => (),
        }
    }

    fn new_tetromino(&mut self) {
        self.grid.store_tetromino(&self.tetromino);
        self.tetromino = self.tetromino_factory.create(&self.config);
        self.block_drop_timer.reset();
    }

    fn handle_input(&mut self, input: &Input) {
        if let Input::Press(ref button) = *input {
            match *button {
                Button::Keyboard(Key::Z) => {
                    match self.tetromino.can_rotate(Rotation::CounterClockwise, &self.grid.boxes) {
                        RotationResult::Allow => self.tetromino.rotate(Rotation::CounterClockwise),
                        _ => (),
                    }
                }
                Button::Keyboard(Key::X) => {
                    match self.tetromino.can_rotate(Rotation::Clockwise, &self.grid.boxes) {
                        RotationResult::Allow => self.tetromino.rotate(Rotation::Clockwise),
                        _ => (),
                    }
                }
                Button::Keyboard(Key::Left) => {
                    match self.tetromino.can_move(Direction::West, &self.grid.boxes) {
                        MoveResult::Allow => self.tetromino.move_left(),
                        MoveResult::Blocked => self.new_tetromino(),
                        _ => (),
                    }
                }
                Button::Keyboard(Key::Right) => {
                    match self.tetromino.can_move(Direction::East, &self.grid.boxes) {
                        MoveResult::Allow => self.tetromino.move_right(),
                        MoveResult::Blocked => self.new_tetromino(),
                        _ => (),
                    }
                }
                Button::Keyboard(Key::Down) => {
                    self.drop_current_block(true);
                }
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

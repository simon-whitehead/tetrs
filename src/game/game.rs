use std::cell::Cell;
use std::rc::Rc;

use gfx_device_gl::Factory;
use piston_window::*;
use piston_window::character::CharacterCache;

use game::asset_factory::AssetFactory;
use game::config::{Config, ConfigBuilder};
use game::factory::TetrominoFactory;
use game::grid::Grid;
use game::tetromino::{Direction, MoveResult, Rotation, RotationResult, Tetromino};
use game::timer::{Timer, TimerTickResult};
use game::window::GameWindow;

pub struct Game {
    time: Rc<Cell<f64>>,
    config: Config,
    asset_factory: AssetFactory,
    grid: Grid,
    lockstep_timer: Timer,
    drop_timer: Timer,
    score: u32,
    score_label: ::game::text::Text,
    tetromino: Tetromino,
    tetromino_factory: TetrominoFactory,
}

impl Game {
    pub fn new(gfx_factory: Factory) -> Game {
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
            asset_factory: AssetFactory::new(gfx_factory),
            grid: Grid::new(),
            lockstep_timer: Timer::new(0.5, time.clone()),
            drop_timer: Timer::new(0.5, time.clone()),
            score: 0,
            score_label: ::game::text::Text::new("Score: 0", 16, 320, 29, [0.0, 0.0, 0.0, 1.0]),
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
                if self.drop_timer.elapsed() || force_drop.into().is_some() {
                    self.tetromino.drop_down();
                    self.drop_timer.reset();
                    self.lockstep_timer.reset();
                }
            }
            MoveResult::Blocked => {
                // If its blocked.. first check if our LockStep has elapsed
                if self.lockstep_timer.elapsed() {
                    // Store the tetromino in the grid and create a new tetromino
                    self.new_tetromino();
                    self.lockstep_timer.stop();
                    let multiplier = self.grid.remove_complete_lines(&self.config);
                    self.score = self.score + (100 * multiplier);
                    self.score_label.set_text(format!("Score: {}", self.score));
                }
            }
            _ => (),
        }
    }

    fn new_tetromino(&mut self) {
        self.grid.store_tetromino(&self.tetromino);
        self.tetromino = self.tetromino_factory.create(&self.config);
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

    pub fn render(&mut self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);

            self.grid.render(&self.config, c, g, &e);
            self.score_label.render(self.asset_factory.font.as_mut().unwrap(), c, g);
        });
    }
}

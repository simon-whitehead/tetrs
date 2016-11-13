use std::cell::Cell;
use std::rc::Rc;

use gfx_device_gl::Factory;
use piston_window::*;

use game::asset_factory::AssetFactory;
use game::config::Config;
use game::factory::TetrominoFactory;
use game::grid::Grid;
use game::render_options::RenderOptions;
use game::scenes::{Scene, SceneResult};
use game::tetromino::{Direction, MoveResult, Rotation, RotationResult, Tetromino};
use game::timer::Timer;
use game::window::GameWindow;

use game::leveling::{Level, LevelingSystem, DefaultLevelingSystem, LevelMetaData};
use game::scoring::{Score, ScoringSystem, ScoreMetaData, DefaultScoringSystem};

static LOCK_STEP_TIME: f64 = 0.5;

static DROP_TIME: f64 = 0.5;
static DROP_FACTOR: f64 = 0.1;

pub struct Game {
    time: Rc<Cell<f64>>,
    config: Config,
    asset_factory: AssetFactory,
    quit: bool,
    grid: Grid,
    lockstep_timer: Timer,
    drop_timer: Timer,
    scoring_system: Box<ScoringSystem>,
    score: Score,
    leveling_system: Box<LevelingSystem>,
    level: Level,
    total_lines_cleared: u32,
    tetromino: Tetromino,
    tetromino_factory: TetrominoFactory,
}

impl Scene for Game {
    fn process(&mut self, e: &Event) -> SceneResult {
        match *e {
            Event::Update(update) => {
                // Update the global game time
                self.update_time(update.dt);

                // Drop the current block if it needs dropping
                if let SceneResult::GameOver = self.move_down(None) {
                    return SceneResult::GameOver;
                }

                // Apply the currently active tetromino into the grid
                self.grid.apply_tetromino(&self.tetromino, &self.config);
            }
            Event::Input(ref input_event) => {
                self.handle_input(input_event);
            }
            _ => (),
        }

        if self.quit {
            SceneResult::MainMenu
        } else {
            SceneResult::None
        }
    }

    fn render(&mut self, window: &mut GameWindow, e: &Event) {
        window.draw_2d(e, |mut c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            let mut options = RenderOptions {
                config: &self.config,
                context: &mut c,
                graphics: g,
                character_cache: self.asset_factory.font.as_mut().unwrap(),
            };

            self.grid.render(&mut options, e);
            self.score.render(&mut options);
            self.level.render(&mut options);
        });
    }
}

impl Game {
    pub fn new(config: Config, gfx_factory: Factory) -> Game {
        let time = RcCell!(0.0);
        let factory = TetrominoFactory::new();

        let tetromino = factory.create(&config);
        let score = Score::new(&config);
        let level = Level::new(&config);

        Game {
            time: time.clone(),
            config: config,
            asset_factory: AssetFactory::new(gfx_factory),
            quit: false,
            grid: Grid::new(),
            lockstep_timer: Timer::new(LOCK_STEP_TIME, time.clone()),
            drop_timer: Timer::new(DROP_TIME, time.clone()),
            score: score,
            level: level,
            scoring_system: Box::new(DefaultScoringSystem),
            leveling_system: Box::new(DefaultLevelingSystem),
            total_lines_cleared: 0,
            tetromino: tetromino,
            tetromino_factory: factory,
        }
    }


    /// Increments the global time
    fn update_time(&mut self, delta: f64) {
        self.time.set(self.time.get() + delta);
    }

    fn move_down<O>(&mut self, force_drop: O) -> SceneResult
        where O: Into<Option<bool>>
    {
        // If we can drop, check if we are ready and drop the active tetromino
        match self.tetromino.can_move(Direction::South, &self.grid.boxes) {
            MoveResult::Allow => {
                if self.drop_timer.elapsed() || force_drop.into().is_some() {
                    self.tetromino.drop_down();
                    self.drop_timer.reset(None);
                    self.lockstep_timer.reset(None);
                }
            }
            MoveResult::Blocked => {
                if self.tetromino.y <= 0 {
                    // We've hit the top
                    return SceneResult::GameOver;
                } else {
                    self.handle_blocked(None);
                }
            }
            _ => (),
        }

        SceneResult::None
    }

    fn handle_blocked<O>(&mut self, force: O)
        where O: Into<Option<bool>>
    {
        // If its blocked.. first check if our LockStep has elapsed
        if self.lockstep_timer.elapsed() || force.into().is_some() {
            // Store the tetromino in the grid and create a new tetromino
            self.new_tetromino();
            self.lockstep_timer.stop();
            let lines_cleared = self.grid.remove_complete_lines(&self.config);
            self.total_lines_cleared += lines_cleared;

            {
                let mut level_metadata = LevelMetaData {
                    level: &mut self.level,
                    lines_cleared: lines_cleared,
                    total_lines_cleared: self.total_lines_cleared,
                };

                self.scoring_system.update_score(&mut self.score,
                                                 ScoreMetaData { lines_cleared: lines_cleared });

                self.leveling_system.process(&mut level_metadata);
            }
            self.drop_timer.reset(DROP_TIME - (self.level.level as f64 * DROP_FACTOR));
        }
    }

    /// Drops a Tetromino straight down until it hits the lowest
    /// possible point.
    fn drop_tetromino(&mut self) {
        while let MoveResult::Allow = self.tetromino.can_move(Direction::South, &self.grid.boxes) {
            self.tetromino.drop_down();
        }
    }

    fn new_tetromino(&mut self) {
        self.grid.store_tetromino(&self.tetromino);
        self.tetromino = self.tetromino_factory.create(&self.config);
    }

    fn handle_input(&mut self, input: &Input) {
        if let Input::Press(ref button) = *input {
            match *button {
                Button::Keyboard(Key::Escape) => self.quit = true,
                Button::Keyboard(Key::Z) => {
                    if let RotationResult::Allow = self.tetromino
                        .can_rotate(Rotation::CounterClockwise, &self.grid.boxes) {
                        self.tetromino.rotate(Rotation::CounterClockwise);
                    }
                }
                Button::Keyboard(Key::X) => {
                    if let RotationResult::Allow = self.tetromino
                        .can_rotate(Rotation::Clockwise, &self.grid.boxes) {
                        self.tetromino.rotate(Rotation::Clockwise);
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
                    self.move_down(true);
                }
                Button::Keyboard(Key::Space) => {
                    self.drop_tetromino();
                    // Stop the lockstep timer straight away
                    self.handle_blocked(true);
                }
                _ => (),
            }
        }
    }
}

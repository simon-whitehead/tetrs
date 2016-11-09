extern crate find_folder;
extern crate rand;

#[macro_use]
mod macros;

mod asset_factory;
mod block;
pub mod config;
mod factory;
mod grid;
mod leveling;
mod render_options;
mod tetromino;
mod text;
mod timer;
mod window;

mod scenes;
mod scoring;

pub use self::config::{Config, ConfigBuilder};
pub use self::window::GameWindow;

pub use self::scenes::{MainMenu, GameOver, Game, Scene, SceneResult};

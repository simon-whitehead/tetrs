mod game;

extern crate gfx_device_gl;
extern crate piston_window;

use piston_window::*;

use game::{Config, ConfigBuilder, MainMenu, Game, GameWindow, Scene, SceneResult};

fn main() {
    let mut window = GameWindow::new(450, 600, "TetRS");
    let config = ConfigBuilder::new()
        .grid_size((10, 22))
        .grid_offset(10.0)
        .tile_size(29.0)
        .build();

    let mut scene: Box<Scene> = Box::new(MainMenu::new(config.clone(),
                                                       window.piston_window.factory.clone()));

    while let Some(e) = window.next() {

        match scene.process(&e) {
            SceneResult::NewGame => {
                scene = Box::new(Game::new(config.clone(), window.piston_window.factory.clone()))
            }
            SceneResult::Quit => break,
            _ => (),
        }

        scene.render(&mut window, &e);
    }
}

mod game;

extern crate gfx_device_gl;
extern crate piston_window;

use piston_window::*;

use game::{Config, ConfigBuilder, MainMenu, Game, GameOver, GameWindow, Scene, SceneResult};

fn main() {
    let mut window = GameWindow::new(450, 600, "TetRS");
    let config = ConfigBuilder::new()
        .grid_size((10, 22))
        .grid_offset(10.0)
        .tile_size(29.0)
        .build();

    let main_menu_generator = |window: &GameWindow, config: &Config| -> MainMenu {
        MainMenu::new(config.clone(), window.piston_window.factory.clone())
    };

    let mut scene: Box<Scene> = Box::new(main_menu_generator(&window, &config));

    while let Some(e) = window.next() {

        match scene.process(&e) {
            SceneResult::MainMenu => scene = Box::new(main_menu_generator(&window, &config)),
            SceneResult::NewGame => {
                scene = Box::new(Game::new(config.clone(), window.piston_window.factory.clone()))
            }
            SceneResult::GameOver => {
                scene = Box::new(GameOver::new(config.clone(),
                                               window.piston_window.factory.clone()))
            }
            SceneResult::Quit => break,
            _ => (),
        }

        scene.render(&mut window, &e);
    }
}

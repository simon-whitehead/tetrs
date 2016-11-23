#[macro_use]
mod game;

use std::cell::{RefCell, RefMut};

extern crate gfx_device_gl;
extern crate piston_window;

use game::{Config, ConfigBuilder, Menu, MenuResult, Game, GameOver, GameWindow, Scene, SceneResult};

fn main() {
    let shadow_arg = std::env::args().nth(1);
    let shadow_enabled = shadow_arg.is_some() && shadow_arg.unwrap() == "--shadow-enabled";

    let mut window = GameWindow::new(450, 600, "TetRS");
    let config = ConfigBuilder::new()
        .grid_size((10, 22))
        .grid_offset(10.0)
        .tile_size(29.0)
        .shadow(shadow_enabled)
        .ui_color([1.0; 4])
        .build();

    let main_menu = RefCell::new(create_main_menu(config, &window));
    let pause_menu = RefCell::new(create_pause_menu(config, &window));

    let game = RefCell::new(Game::new(config, window.piston_window.factory.clone()));
    let gameover = RefCell::new(GameOver::new(config, window.piston_window.factory.clone()));

    let mut scene: RefMut<Scene> = main_menu.borrow_mut();

    while let Some(e) = window.next() {
        match scene.process(&e) {
            SceneResult::MainMenu => {
                scene = main_menu.borrow_mut();
            }
            SceneResult::NewGame => {
                game.borrow_mut().reset();
                scene = game.borrow_mut();
            }
            SceneResult::PauseGame => {
                scene = pause_menu.borrow_mut();
            }
            SceneResult::ResumeGame => {
                game.borrow_mut().unpause();
                scene = game.borrow_mut();
            }
            SceneResult::GameOver => {
                scene = gameover.borrow_mut();
            }
            SceneResult::Quit => break,
            _ => (),
        };

        scene.render(&mut window, &e);
    }
}

fn create_main_menu(config: Config, window: &GameWindow) -> Menu {
    menu![
        (config, window.piston_window.factory.clone()),
        "New Game" => MenuResult::NewGame,
        "Quit" => MenuResult::Quit
    ]
}

fn create_pause_menu(config: Config, window: &GameWindow) -> Menu {
    menu![
        (config, window.piston_window.factory.clone()),
        "Resume" => MenuResult::ResumeGame,
        "New Game" => MenuResult::NewGame,
        "Main Manu" => MenuResult::MainMenu
    ]
}
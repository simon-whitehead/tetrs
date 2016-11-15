mod game;

use std::cell::{RefCell, RefMut};

extern crate gfx_device_gl;
extern crate piston_window;

use game::{Config, ConfigBuilder, Menu, MenuResult, Game, GameOver, GameWindow, Scene, SceneResult};

fn main() {
    let mut window = GameWindow::new(450, 600, "TetRS");
    let config = ConfigBuilder::new()
        .grid_size((10, 22))
        .grid_offset(10.0)
        .tile_size(29.0)
        .shadow(false)
        .ui_color([1.0; 4])
        .build();

    let main_menu = create_main_menu(config, &window);
    let pause_menu = create_pause_menu(config, &window);

    let game = RefCell::new(Game::new(config, window.piston_window.factory.clone()));
    let gameover = RefCell::new(GameOver::new(config, window.piston_window.factory.clone()));

    let mut scene: RefMut<Scene> = main_menu.borrow_mut();

    while let Some(e) = window.next() {

        let scene_result = scene.process(&e);

        match scene_result {
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

fn create_main_menu(config: Config, window: &GameWindow) -> RefCell<Menu> {
    let mut main_menu = Menu::new(config, window.piston_window.factory.clone());
    main_menu.add_item("New Game", MenuResult::NewGame);
    main_menu.add_item("Quit", MenuResult::Quit);

    RefCell::new(main_menu)
}

fn create_pause_menu(config: Config, window: &GameWindow) -> RefCell<Menu> {
    let mut pause_menu = Menu::new(config, window.piston_window.factory.clone());
    pause_menu.add_item("Resume", MenuResult::ResumeGame);
    pause_menu.add_item("New Game", MenuResult::NewGame);
    pause_menu.add_item("Main Menu", MenuResult::MainMenu);

    RefCell::new(pause_menu)
}
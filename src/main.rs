mod game;

extern crate gfx_device_gl;
extern crate piston_window;

use piston_window::*;

use game::{MainMenu, Game, GameWindow, Scene, SceneResult};

fn main() {
    let mut window = GameWindow::new(450, 600, "TetRS");
    let mut scene: Box<Scene> = Box::new(MainMenu::new(window.piston_window.factory.clone()));

    while let Some(e) = window.next() {

        match scene.process(&e) {
            SceneResult::NewGame => {
                scene = Box::new(Game::new(window.piston_window.factory.clone()))
            }
            SceneResult::Quit => break,
            _ => (),
        }

        scene.render(&mut window, &e);
    }
}

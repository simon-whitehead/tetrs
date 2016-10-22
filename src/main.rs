mod game;
mod engine;

extern crate piston_window;

use piston_window::*;

use game::Game;
use engine::GameWindow;

fn main() {
    let mut window = GameWindow::new(800, 600, "TetRS");
    let mut game = Game::new();

    while let Some(e) = window.next() {

        if !game.process(&e) {
            break;
        }

        game.render(&mut window, &e);
    }
}

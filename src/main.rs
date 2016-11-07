mod game;

extern crate gfx_device_gl;
extern crate piston_window;

use piston_window::*;

use game::{Game, GameWindow};

fn main() {
    let mut window = GameWindow::new(450, 600, "TetRS");
    let mut game = Game::new(window.piston_window.factory.clone());

    while let Some(e) = window.next() {

        if !game.process(&e) {
            break;
        }

        game.render(&mut window, &e);
    }
}

use piston_window::*;

use game::window::GameWindow;

pub trait Scene {
    fn process(&mut self, e: &Event) -> SceneResult;
    fn render(&mut self, window: &mut GameWindow, e: &Event);
}

pub enum SceneResult {
    None,
    NewGame,
    GameOver,
    Quit,
}
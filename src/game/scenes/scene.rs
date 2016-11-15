use piston_window::*;

use game::scenes::MenuResult;
use game::window::GameWindow;

pub trait Scene {
    fn process(&mut self, e: &Event) -> SceneResult;
    fn render(&mut self, window: &mut GameWindow, e: &Event);
}

pub enum SceneResult {
    None,
    MainMenu,
    NewGame,
    PauseGame,
    ResumeGame,
    GameOver,
    Quit,
}

impl From<MenuResult> for SceneResult {
    fn from(menu: MenuResult) -> SceneResult {
        match menu {
            MenuResult::MainMenu => SceneResult::MainMenu,
            MenuResult::NewGame => SceneResult::NewGame,
            MenuResult::ResumeGame => SceneResult::ResumeGame,
            MenuResult::Quit => SceneResult::Quit,
            _ => SceneResult::None,
        }
    }
}
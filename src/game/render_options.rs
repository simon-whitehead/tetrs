
use piston_window::*;
use piston_window::character::CharacterCache;

use game::config::Config;

pub struct RenderOptions<'a, G: 'a, C: 'a>
    where C: CharacterCache,
          G: Graphics<Texture = <C as CharacterCache>::Texture>
{
    pub config: &'a Config,
    pub character_cache: &'a mut C,
    pub context: &'a mut Context,
    pub graphics: &'a mut G,
}
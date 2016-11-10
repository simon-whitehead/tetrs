use gfx_device_gl::Factory;
use piston_window::*;

use game::find_folder::Search;

pub struct AssetFactory {
    pub font: Option<Glyphs>,
}

impl AssetFactory {
    pub fn new(factory: Factory) -> AssetFactory {
        let asset_folder = Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let font = asset_folder.join("NimbusSanL-Regu.ttf");

        let glyphs = Glyphs::new(&font, factory.clone()).unwrap();

        AssetFactory { font: Some(glyphs) }
    }
}
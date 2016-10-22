
use piston_window::*;

use ::engine::GameWindow;
use ::game::block::Block;

const GRID_OFFSET: f64 = 20.0;
const TILE_SIZE: f64 = 29.0;

pub struct Grid {
    boxes: [[Block; 10]; 22],
}

impl Grid {
    pub fn new() -> Grid {
        Grid { boxes: [[Block::new([0.0, 0.0, 0.0, 1.0]); 10]; 22] }
    }

    pub fn render<G>(&self, context: Context, gfx: &mut G, e: &Event)
        where G: Graphics
    {
        for mut y in 2..22 {
            for mut x in 0..10 {
                let adjusted_y = y - 2;
                rectangle(self.boxes[y][x].color,
                          [x as f64 * TILE_SIZE + 10.0,
                           adjusted_y as f64 * TILE_SIZE + 10.0,
                           TILE_SIZE as f64,
                           TILE_SIZE as f64],
                          context.transform,
                          gfx);
            }
        }
    }
}
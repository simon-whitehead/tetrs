
use piston_window::*;

use ::game::block::Block;
use ::game::config::Config;
use ::game::tetromino::Tetromino;
use ::game::window::GameWindow;

pub struct Grid {
    pub boxes: [[Option<Block>; 10]; 22],
    overlay: [[Option<Block>; 10]; 22],
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            boxes: [[None; 10]; 22],
            overlay: [[None; 10]; 22],
        }
    }

    pub fn apply_tetromino(&mut self, tetromino: &Tetromino) {
        self.overlay = [[None; 10]; 22];

        for y in 0..4 {
            for x in 0..4 {
                if let Some(ref block) = tetromino.blocks[y][x] {
                    let x = (tetromino.x + x as i32) as usize;
                    let y = (tetromino.y + y as i32) as usize;

                    if x >= 10 || y >= 22 {
                        continue;
                    }

                    self.overlay[y][x] = Some(block.clone());
                }
            }
        }
    }

    pub fn store_tetromino(&mut self, tetromino: &Tetromino) {
        for y in 0..4 {
            for x in 0..4 {
                if let Some(ref block) = tetromino.blocks[y][x] {
                    let x = (tetromino.x + x as i32) as usize;
                    let y = (tetromino.y + y as i32) as usize;

                    if x >= 10 || y >= 22 {
                        continue;
                    }

                    self.boxes[y][x] = Some(block.clone());
                }
            }
        }
    }

    pub fn render<G>(&self, config: &Config, context: Context, gfx: &mut G, e: &Event)
        where G: Graphics
    {
        let grid_offset = config.grid_offset;
        let tile_size = config.tile_size;

        for mut y in 2..22 {
            for mut x in 0..10 {

                let adjusted_y = y - 2;
                if let Some(ref block) = self.overlay[y][x] {
                    rectangle(block.color,
                              [x as f64 * tile_size + grid_offset,
                               adjusted_y as f64 * tile_size + grid_offset,
                               tile_size as f64,
                               tile_size as f64],
                              context.transform,
                              gfx);
                    continue;
                }

                let color = match self.boxes[y][x] {
                    Some(ref block) => block.color,
                    None => [0.0, 0.0, 0.0, 1.0],
                };
                rectangle(color,
                          [x as f64 * tile_size + grid_offset,
                           adjusted_y as f64 * tile_size + grid_offset,
                           tile_size as f64,
                           tile_size as f64],
                          context.transform,
                          gfx);
            }
        }
    }
}

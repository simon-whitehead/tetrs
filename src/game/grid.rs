
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

    /// Removes complete lines from the grid
    pub fn remove_complete_lines(&mut self, config: &Config) -> u32 {
        // Pointer to where we're currently writing lines
        let mut y_mut = (config.grid_size.1 - 1) as usize;
        let mut cleared_lines = 0;

        for y in (0..config.grid_size.1).rev() {
            // If every column in this row has a block,
            // we consider it "complete"
            let complete = self.boxes[y as usize].iter().all(|block| block.is_some());

            // If this is not a complete line, copy it
            // into the currently pointed line
            if !complete {
                self.boxes[y_mut] = self.boxes[y as usize];
                y_mut = y_mut - 1;
                if y_mut == 0 {
                    break;
                }
            } else {
                cleared_lines = cleared_lines + 1;
            }
        }

        cleared_lines
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
                    block.render(x, y, &config, context, gfx, e);
                    continue;
                }

                match self.boxes[y][x] {
                    Some(ref block) => {
                        block.render(x, y, &config, context, gfx, e);
                    }
                    None => {
                        rectangle([0.0, 0.0, 0.0, 1.0],
                                  [x as f64 * tile_size + grid_offset,
                                   adjusted_y as f64 * tile_size + grid_offset,
                                   tile_size as f64,
                                   tile_size as f64],
                                  context.transform,
                                  gfx);
                    }
                };
            }
        }
    }
}


use piston_window::*;
use piston_window::character::CharacterCache;

use game::block::Block;
use game::config::Config;
use game::render_options::RenderOptions;
use game::tetromino::Tetromino;

pub struct Grid {
    pub boxes: [[Option<Block>; 10]; 22],
    overlay: [[Option<Block>; 10]; 22],
    border: rectangle::Rectangle,
    block: rectangle::Rectangle,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            boxes: [[None; 10]; 22],
            overlay: [[None; 10]; 22],
            border: rectangle::Rectangle::new([1.0; 4]),
            block: rectangle::Rectangle::new([0.0, 0.0, 0.0, 1.0]),
        }
    }

    pub fn apply_tetromino(&mut self, tetromino: &Tetromino, config: &Config) {
        // Clear the overlay
        self.overlay = [[None; 10]; 22];

        // Iterate over the blocks in the active Tetromino and apply them
        // to the overlay
        for y in 0..4 {
            for x in 0..4 {
                if let Some(ref block) = tetromino.blocks[y][x] {
                    let x = (tetromino.x + x as i32) as usize;
                    let y = (tetromino.y + y as i32) as usize;

                    if x >= 10 || y >= 22 {
                        continue;
                    }

                    self.overlay[y][x] = Some(*block);

                    // If the shadow is enabled, also store that in the overlay
                    if config.shadow_enabled {
                        let (shadow_x, shadow_y) = tetromino.find_landing_xy(&self.boxes);
                        let shadow_x = (shadow_x + x as i32) as usize;
                        let shadow_y = (shadow_y + y as i32) as usize;

                        if let Some(ref shadow) = tetromino.shadow[y][x] {
                            self.overlay[shadow_y][shadow_x] = Some(*shadow);
                        }
                    }
                }
            }
        }
    }

    /// Permanently stores the Tetromino in the grid
    pub fn store_tetromino(&mut self, tetromino: &Tetromino) {
        for y in 0..4 {
            for x in 0..4 {
                if let Some(ref block) = tetromino.blocks[y][x] {
                    let x = (tetromino.x + x as i32) as usize;
                    let y = (tetromino.y + y as i32) as usize;

                    if x >= 10 || y >= 22 {
                        continue;
                    }

                    self.boxes[y][x] = Some(*block);
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
                y_mut -= 1;
                if y_mut == 0 {
                    break;
                }
            } else {
                cleared_lines += 1;
            }
        }

        cleared_lines
    }

    pub fn render<'a, G, C>(&self, options: &mut RenderOptions<'a, G, C>, e: &Event)
        where C: CharacterCache,
              G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        let grid_offset = options.config.grid_offset;
        let tile_size = options.config.tile_size;

        // Draw the "border" first
        self.border.draw([(grid_offset - 2.0) as f64,
                          (grid_offset - 2.0) as f64,
                          (tile_size * 10.0 + 4.0) as f64,
                          (tile_size * 20.0 + 4.0) as f64],
                         &Default::default(),
                         options.context.transform,
                         options.graphics);

        for y in 2..options.config.grid_size.1 {
            for x in 0..options.config.grid_size.0 {
                let x = x as usize;
                let y = y as usize;

                let adjusted_y = y - 2;
                if let Some(ref block) = self.overlay[y][x] {
                    block.render(x, y, options.config, options.context, options.graphics, e);
                    continue;
                }

                match self.boxes[y as usize][x as usize] {
                    Some(ref block) => {
                        block.render(x, y, options.config, options.context, options.graphics, e);
                    }
                    None => {
                        self.block.draw([x as f64 * tile_size + grid_offset,
                                         adjusted_y as f64 * tile_size + grid_offset,
                                         tile_size as f64,
                                         tile_size as f64],
                                        &Default::default(),
                                        options.context.transform,
                                        options.graphics);
                    }
                };
            }
        }
    }
}

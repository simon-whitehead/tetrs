use piston_window::*;

use game::config::Config;

#[derive(Copy, Clone)]
pub struct Block {
    pub color: [f32; 4],
}

impl Block {
    pub fn new(color: [f32; 4]) -> Block {
        Block { color: color }
    }

    pub fn render<G>(&self,
                     x: usize,
                     y: usize,
                     config: &Config,
                     context: Context,
                     gfx: &mut G,
                     e: &Event)
        where G: Graphics
    {
        let grid_offset = config.grid_offset;
        let tile_size = config.tile_size;
        let adjusted_y = y - 2;

        let rect = rectangle::Rectangle {
            color: self.color,
            shape: rectangle::Shape::Bevel(1.0),
            border: Some(rectangle::Border {
                color: [0.0, 0.0, 0.0, 1.0],
                radius: 1.0,
            }),
        };

        rect.draw([x as f64 * tile_size + grid_offset,
                   adjusted_y as f64 * tile_size + grid_offset,
                   tile_size as f64,
                   tile_size as f64],
                  &Default::default(),
                  context.transform,
                  gfx);
    }
}

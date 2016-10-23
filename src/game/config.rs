
pub struct Config {
    pub grid_offset: f64,
    pub tile_size: f64,
}

pub struct ConfigBuilder {
    grid_offset: f64,
    tile_size: f64,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            grid_offset: 0.0,
            tile_size: 0.0,
        }
    }

    pub fn grid_offset(mut self, grid_offset: f64) -> Self {
        self.grid_offset = grid_offset;
        self
    }

    pub fn tile_size(mut self, tile_size: f64) -> Self {
        self.tile_size = tile_size;
        self
    }

    pub fn build(mut self) -> Config {
        Config {
            grid_offset: self.grid_offset,
            tile_size: self.tile_size,
        }
    }
}
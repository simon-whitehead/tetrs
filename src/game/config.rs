
pub struct Config {
    pub grid_offset: f64,
    pub tile_size: f64,
    pub grid_size: (u32, u32),
    pub shadow_enabled: bool,
}

pub struct ConfigBuilder {
    g_size: (u32, u32),
    g_offset: f64,
    t_size: f64,
    shadow_enabled: bool,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            g_size: (0, 0),
            g_offset: 0.0,
            t_size: 0.0,
            shadow_enabled: false,
        }
    }

    pub fn grid_size(mut self, grid_size: (u32, u32)) -> Self {
        self.g_size = grid_size;
        self
    }

    pub fn grid_offset(mut self, grid_offset: f64) -> Self {
        self.g_offset = grid_offset;
        self
    }

    pub fn tile_size(mut self, tile_size: f64) -> Self {
        self.t_size = tile_size;
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow_enabled = shadow;
        self
    }

    pub fn build(mut self) -> Config {
        Config {
            grid_size: self.g_size,
            grid_offset: self.g_offset,
            tile_size: self.t_size,
            shadow_enabled: self.shadow_enabled,
        }
    }
}


#[derive(Copy, Clone)]
pub struct Config {
    pub grid_offset: f64,
    pub tile_size: f64,
    pub grid_size: (u32, u32),
    pub ui_color: [f32; 4],
    pub shadow_enabled: bool,
}

pub struct ConfigBuilder {
    g_size: (u32, u32),
    g_offset: f64,
    t_size: f64,
    ui_color: [f32; 4],
    shadow_enabled: bool,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            g_size: (0, 0),
            g_offset: 0.0,
            t_size: 0.0,
            ui_color: [1.0; 4],
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

    pub fn ui_color(mut self, ui_color: [f32; 4]) -> Self {
        self.ui_color = ui_color;
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow_enabled = shadow;
        self
    }

    pub fn build(self) -> Config {
        Config {
            grid_size: self.g_size,
            grid_offset: self.g_offset,
            tile_size: self.t_size,
            ui_color: self.ui_color,
            shadow_enabled: self.shadow_enabled,
        }
    }
}

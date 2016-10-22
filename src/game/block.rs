#[derive(Copy, Clone)]
pub struct Block {
    pub color: [f32; 4],
}

impl Block {
    pub fn new(color: [f32; 4]) -> Block {
        Block { color: color }
    }
}
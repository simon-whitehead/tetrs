use piston_window::*;

use ::game::block::Block;
use ::game::config::Config;

#[derive(Copy, Clone)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub struct Tetromino {
    pub x: i32,
    pub y: i32,
    pub blocks: [[Option<Block>; 4]; 4],
    shape: Shape,
    direction: Direction,
}

pub struct TetrominoShape(Shape, [[Option<Block>; 4]; 4]);

impl Tetromino {
    pub fn new(shape: TetrominoShape, config: &Config) -> Tetromino {
        Tetromino {
            x: (config.grid_size.0 as i32 / 2) - 2,
            y: 0,
            shape: shape.0,
            blocks: shape.1,
            direction: Direction::North,
        }
    }

    pub fn drop_down(&mut self, config: &Config) -> bool {
        let bottom = self.get_bottom_block_index();

        if self.y + bottom < config.grid_size.1 as i32 - 1 {
            self.y = self.y + 1;
            true
        } else {
            false
        }
    }

    pub fn move_up(&mut self) {
        self.y = self.y - 1;
    }

    pub fn move_left(&mut self) {
        let left = self.get_leftmost_block_index();
        if self.x + left > 0 {
            self.x = self.x - 1;
        }
    }

    pub fn move_right(&mut self, right_side: u32) {
        let right = self.get_rightmost_block_index();
        if self.x + right < right_side as i32 - 1 {
            self.x = self.x + 1;
        }
    }

    fn get_leftmost_block_index(&self) -> i32 {
        for x in (0..4) {
            for y in (0..4) {
                if let Some(ref block) = self.blocks[y][x] {
                    return x as i32;
                }
            }
        }

        0
    }

    fn get_rightmost_block_index(&self) -> i32 {
        for x in (0..4).rev() {
            for y in (0..4).rev() {
                if let Some(ref block) = self.blocks[y][x] {
                    return x as i32;
                }
            }
        }

        3
    }

    fn get_bottom_block_index(&self) -> i32 {
        for y in (0..4).rev() {
            for x in (0..4).rev() {
                if let Some(ref block) = self.blocks[y][x] {
                    return x as i32;
                }
            }
        }

        3
    }
}

pub struct TetrominoFactory {
    shapes: Vec<(Shape, [[Option<Block>; 4]; 4])>,
}

impl TetrominoFactory {
    pub fn new() -> TetrominoFactory {
        let mut shapes = vec![
            Self::create_I(),
            Self::create_J(),
            Self::create_L(),
            Self::create_O(),
            Self::create_S(),
            Self::create_T(),
            Self::create_Z(),
        ];

        TetrominoFactory { shapes: shapes }
    }

    pub fn create(&self, config: &Config) -> Tetromino {
        let shape = Self::create_S();
        Tetromino::new(TetrominoShape(shape.0, shape.1), &config)
    }

    fn create_I() -> (Shape, [[Option<Block>; 4]; 4]) {
        let i_color = [0.0, 1.0, 1.0, 1.0];
        let i = Some(Block::new(i_color));

        (Shape::I,
         [[None, i.clone(), None, None],
          [None, i.clone(), None, None],
          [None, i.clone(), None, None],
          [None, i.clone(), None, None]])
    }

    fn create_J() -> (Shape, [[Option<Block>; 4]; 4]) {
        let j_color = [0.0, 0.0, 1.0, 1.0];
        let j = Some(Block::new(j_color));

        (Shape::J,
         [[None, j.clone(), None, None],
          [None, j.clone(), None, None],
          [j.clone(), j.clone(), None, None],
          [None, None, None, None]])
    }

    fn create_L() -> (Shape, [[Option<Block>; 4]; 4]) {
        let l_color = [0.8, 0.5, 0.0, 1.0];
        let l = Some(Block::new(l_color));

        (Shape::L,
         [[None, l.clone(), None, None],
          [None, l.clone(), None, None],
          [None, l.clone(), l.clone(), None],
          [None, None, None, None]])
    }

    fn create_O() -> (Shape, [[Option<Block>; 4]; 4]) {
        let o_color = [1.0, 1.0, 0.0, 1.0];
        let o = Some(Block::new(o_color));

        (Shape::O,
         [[None, o.clone(), o.clone(), None],
          [None, o.clone(), o.clone(), None],
          [None, None, None, None],
          [None, None, None, None]])
    }

    fn create_S() -> (Shape, [[Option<Block>; 4]; 4]) {
        let s_color = [0.0, 1.0, 0.0, 1.0];
        let s = Some(Block::new(s_color));

        (Shape::S,
         [[None, s.clone(), s.clone(), None],
          [s.clone(), s.clone(), None, None],
          [None, None, None, None],
          [None, None, None, None]])
    }

    fn create_T() -> (Shape, [[Option<Block>; 4]; 4]) {
        let t_color = [1.0, 0.4, 0.7, 1.0];
        let t = Some(Block::new(t_color));

        (Shape::T,
         [[None, t.clone(), None, None],
          [t.clone(), t.clone(), t.clone(), None],
          [None, None, None, None],
          [None, None, None, None]])
    }

    fn create_Z() -> (Shape, [[Option<Block>; 4]; 4]) {
        let z_color = [1.0, 0.0, 0.0, 1.0];
        let z = Some(Block::new(z_color));

        (Shape::Z,
         [[z.clone(), z.clone(), None, None],
          [None, z.clone(), z.clone(), None],
          [None, None, None, None],
          [None, None, None, None]])
    }
}
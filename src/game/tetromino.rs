use piston_window::*;

use ::game::block::Block;
use ::game::config::Config;

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub enum MoveResult {
    Allow,
    Deny,
    Blocked,
}

#[derive(Copy, Clone)]
pub struct Tetromino {
    pub x: i32,
    pub y: i32,
    pub blocks: [[Option<Block>; 4]; 4],
    north: [[Option<Block>; 4]; 4],
    east: [[Option<Block>; 4]; 4],
    south: [[Option<Block>; 4]; 4],
    west: [[Option<Block>; 4]; 4],
    direction: Direction,
}

pub struct TetrominoShape([[Option<Block>; 4]; 4],[[Option<Block>; 4]; 4],[[Option<Block>; 4]; 4],[[Option<Block>; 4]; 4]);

impl Tetromino {
    pub fn new(shape: TetrominoShape, config: &Config) -> Tetromino {
        Tetromino {
            x: (config.grid_size.0 as i32 / 2) - 2,
            y: 0,
            blocks: shape.0.clone(),
            north: shape.0,
            east: shape.1,
            south: shape.2,
            west: shape.3,
            direction: Direction::North,
        }
    }

    /// Checks if the current block can move in a specific
    /// direction.
    pub fn can_move(&self, direction: Direction, grid: &[[Option<Block>; 10]; 22]) -> MoveResult {

        // Determine the direction on each axis we're attempting to move
        let mut y_dir = Self::get_y_direction(direction) as i32;
        let mut x_dir = Self::get_x_direction(direction) as i32;

        // Loop over each block of this tetromino and compare it
        // to the offset within the grid where we want to move to
        for y in 0..4 {
            for x in 0..4 {
                if let Some(ref block) = self.blocks[y][x] {
                    let x = ((self.x + x as i32) + x_dir) as isize;
                    let y = ((self.y + y as i32) + y_dir) as isize;

                    // Check if we've hit the bottom
                    if y > 21 {
                        return MoveResult::Blocked;
                    }

                    // Check if we're touching the edges
                    if x < 0 || x > 9 {
                        return MoveResult::Deny;
                    }

                    // Otherwise check if we're smashing in to another block
                    if let Some(ref block) = grid[y as usize][x as usize] {
                        // Deny left and right.. but block downwards
                        match direction {
                            Direction::East | Direction::West => return MoveResult::Deny,
                            Direction::North | Direction::South => return MoveResult::Blocked,
                        }
                    }
                }
            }
        }

        MoveResult::Allow
    }

    fn get_y_direction(direction: Direction) -> isize {
        match direction {
            Direction::North => -1,
            Direction::South => 1,
            _ => 0,
        }
    }

    fn get_x_direction(direction: Direction) -> isize {
        match direction {
            Direction::East => 1,
            Direction::West => -1,
            _ => 0,
        }
    }

    pub fn drop_down(&mut self) {
        self.y = self.y + 1;
    }

    pub fn move_up(&mut self) {
        self.y = self.y - 1;
    }

    pub fn move_left(&mut self) {
        self.x = self.x - 1;
    }

    pub fn move_right(&mut self) {
        self.x = self.x + 1;
    }
}

pub struct TetrominoFactory {
    shapes: Vec<TetrominoShape>,
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
        let shape = Self::create_I();
        Tetromino::new(TetrominoShape(shape.0, shape.1, shape.2, shape.3), &config)
    }

    fn create_I() -> TetrominoShape {
        let i_color = [0.0, 1.0, 1.0, 1.0];
        let i = Some(Block::new(i_color));

        TetrominoShape(
        // North
         [[None, None, None, None],
          [i.clone(), i.clone(), i.clone(), i.clone()],
          [None, None, None, None],
          [None, None, None, None]],
          
          // East
         [[None, None, i.clone(), None],
          [None, None, i.clone(), None],
          [None, None, i.clone(), None],
          [None, None, i.clone(), None]],

          // South
         [[None, None, None, None],
          [None, None, None, None],
          [i.clone(), i.clone(), i.clone(), i.clone()],
          [None, None, None, None]],

          // West
         [[None, i.clone(), None, None],
          [None, i.clone(), None, None],
          [None, i.clone(), None, None],
          [None, i.clone(), None, None]],
        )
    }

    fn create_J() -> TetrominoShape {
        let j_color = [0.0, 0.0, 1.0, 1.0];
        let j = Some(Block::new(j_color));

        TetrominoShape(
        // North
         [[j.clone(), None, None, None],
          [j.clone(), j.clone(), j.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],
          
          // East
         [[None, j.clone(), j.clone(), None],
          [None, j.clone(), None, None],
          [None, j.clone(), None, None],
          [None, None, None, None]],

          // South
         [[None, None, None, None],
          [j.clone(), j.clone(), j.clone(), None],
          [None, None, j.clone(), None],
          [None, None, None, None]],

          // West
         [[None, j.clone(), None, None],
          [None, j.clone(), None, None],
          [j.clone(), j.clone(), None, None],
          [None, None, None, None]],
        )
    }

    fn create_L() -> TetrominoShape {
        let l_color = [0.8, 0.5, 0.0, 1.0];
        let l = Some(Block::new(l_color));

        TetrominoShape(
        // North
         [[None, None, l.clone(), None],
          [l.clone(), l.clone(), l.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],
          
          // East
         [[None, l.clone(), None, None],
          [None, l.clone(), None, None],
          [None, l.clone(), l.clone(), None],
          [None, None, None, None]],

          // South
         [[None, None, None, None],
          [l.clone(), l.clone(), l.clone(), None],
          [l.clone(), None, None, None],
          [None, None, None, None]],

          // West
         [[l.clone(), l.clone(), None, None],
          [None, l.clone(), None, None],
          [None, l.clone(), None, None],
          [None, None, None, None]],
        )
    }

    fn create_O() -> TetrominoShape {
        let o_color = [1.0, 1.0, 0.0, 1.0];
        let o = Some(Block::new(o_color));

        TetrominoShape(
        // North
         [[None, o.clone(), o.clone(), None],
          [None, o.clone(), o.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],
          
          // East
         [[None, o.clone(), o.clone(), None],
          [None, o.clone(), o.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],

          // South
         [[None, o.clone(), o.clone(), None],
          [None, o.clone(), o.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],

          // West
         [[None, o.clone(), o.clone(), None],
          [None, o.clone(), o.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],
        )
    }

    fn create_S() -> TetrominoShape {
        let s_color = [0.0, 1.0, 0.0, 1.0];
        let s = Some(Block::new(s_color));

        TetrominoShape(
        // North
         [[None, s.clone(), s.clone(), None],
          [s.clone(), s.clone(), None, None],
          [None, None, None, None],
          [None, None, None, None]],
          
          // East
         [[None, s.clone(), None, None],
          [None, s.clone(), s.clone(), None],
          [None, None, s.clone(), None],
          [None, None, None, None]],

          // South
         [[None, None, None, None],
          [None, s.clone(), s.clone(), None],
          [s.clone(), s.clone(), None, None],
          [None, None, None, None]],

          // West
         [[s.clone(), None, None, None],
          [s.clone(), s.clone(), None, None],
          [None, s.clone(), None, None],
          [None, None, None, None]],
        )
    }

    fn create_T() -> TetrominoShape {
        let t_color = [1.0, 0.4, 0.7, 1.0];
        let t = Some(Block::new(t_color));

        TetrominoShape(
        // North
         [[None, t.clone(), None, None],
          [t.clone(), t.clone(), t.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],
          
          // East
         [[None, t.clone(), None, None],
          [None, t.clone(), t.clone(), None],
          [None, t.clone(), None, None],
          [None, None, None, None]],

          // South
         [[None, None, None, None],
          [t.clone(), t.clone(), t.clone(), None],
          [None, t.clone(), None, None],
          [None, None, None, None]],

          // West
         [[None, t.clone(), None, None],
          [t.clone(), t.clone(), None, None],
          [None, t.clone(), None, None],
          [None, None, None, None]],
        )
    }

    fn create_Z() -> TetrominoShape {
        let z_color = [1.0, 0.0, 0.0, 1.0];
        let z = Some(Block::new(z_color));

        TetrominoShape(
        // North
         [[z.clone(), z.clone(), None, None],
          [None, z.clone(), z.clone(), None],
          [None, None, None, None],
          [None, None, None, None]],
          
          // East
         [[None, None, None, z.clone()],
          [None, None, z.clone(), z.clone()],
          [None, None, z.clone(), None],
          [None, None, None, None]],

          // South
         [[None, None, None, None],
          [z.clone(), z.clone(), None, None],
          [None, z.clone(), z.clone(), None],
          [None, None, None, None]],

          // West
         [[None, z.clone(), None, None],
          [z.clone(), z.clone(), None, None],
          [z.clone(), None, None, None],
          [None, None, None, None]],
        )
    }
}
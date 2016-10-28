use piston_window::*;

use ::game::block::Block;
use ::game::config::Config;
use game::factory::TetrominoFactory;

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

pub enum Rotation {
    Clockwise,
    CounterClockwise,
}

pub enum RotationResult {
    Allow,
    Deny,
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

pub struct TetrominoShape(pub [[Option<Block>; 4]; 4],
                          pub [[Option<Block>; 4]; 4],
                          pub [[Option<Block>; 4]; 4],
                          pub [[Option<Block>; 4]; 4]);

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

    /// Checks if the current tetromino can rotate in its given position
    pub fn can_rotate(&self,
                      rotation: Rotation,
                      grid: &[[Option<Block>; 10]; 22])
                      -> RotationResult {

        let new_dir = self.get_rotated_position(rotation);
        let desired_blocks = self.get_blocks_for_direction(new_dir);

        // Loop over each block of this tetromino and compare it
        // to the offset within the grid where we want to move to
        for y in 0..4 {
            for x in 0..4 {
                if let Some(ref block) = desired_blocks[y][x] {
                    let x = (self.x + x as i32) as isize;
                    let y = (self.y + y as i32) as isize;

                    // Check if we will hit the bottom
                    if y > 21 {
                        return RotationResult::Deny;
                    }

                    // Check if we might hit the edge
                    if x < 0 || x > 9 {
                        return RotationResult::Deny;
                    }

                    // Otherwise check if we're smashing in to another block
                    if let Some(ref block) = grid[y as usize][x as usize] {
                        // Deny left and right.. but block downwards
                        return RotationResult::Deny;
                    }
                }
            }
        }

        RotationResult::Allow
    }

    // Determines the direction we want to be facing this Tetromino, based
    // on its current direction and the direction the player attempted to
    // turn.
    fn get_rotated_position(&self, direction: Rotation) -> Direction {
        match direction {
            Rotation::Clockwise => {
                match self.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                }
            }
            Rotation::CounterClockwise => {
                match self.direction {
                    Direction::North => Direction::West,
                    Direction::West => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                }
            }
        }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let new_dir = self.get_rotated_position(rotation);
        self.blocks = self.get_blocks_for_direction(new_dir);
        self.direction = new_dir;
    }

    fn get_blocks_for_direction(&self, direction: Direction) -> [[Option<Block>; 4]; 4] {
        match direction {
            Direction::North => self.north.clone(),
            Direction::East => self.east.clone(),
            Direction::South => self.south.clone(),
            Direction::West => self.west.clone(),
        }
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

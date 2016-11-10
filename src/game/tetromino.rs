
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
    pub shadow: [[Option<Block>; 4]; 4],
    north: [[Option<Block>; 4]; 4],
    east: [[Option<Block>; 4]; 4],
    south: [[Option<Block>; 4]; 4],
    west: [[Option<Block>; 4]; 4],
    shadow_north: [[Option<Block>; 4]; 4],
    shadow_east: [[Option<Block>; 4]; 4],
    shadow_south: [[Option<Block>; 4]; 4],
    shadow_west: [[Option<Block>; 4]; 4],
    direction: Direction,
}

pub struct TetrominoShape(pub [[Option<Block>; 4]; 4],
                          pub [[Option<Block>; 4]; 4],
                          pub [[Option<Block>; 4]; 4],
                          pub [[Option<Block>; 4]; 4]);

impl Tetromino {
    pub fn new(shape: TetrominoShape, shadow: TetrominoShape, config: &Config) -> Tetromino {
        Tetromino {
            x: (config.grid_size.0 as i32 / 2) - 2,
            y: 0,
            blocks: shape.0,
            shadow: shadow.0,
            north: shape.0,
            east: shape.1,
            south: shape.2,
            west: shape.3,
            shadow_north: shadow.0,
            shadow_east: shadow.1,
            shadow_south: shadow.2,
            shadow_west: shadow.3,
            direction: Direction::North,
        }
    }

    /// Checks if the current block can move in a specific
    /// direction.
    pub fn can_move(&self, direction: Direction, grid: &[[Option<Block>; 10]; 22]) -> MoveResult {

        // Determine the direction on each axis we're attempting to move
        let y_dir = Self::get_y_direction(direction) as i32;
        let x_dir = Self::get_x_direction(direction) as i32;

        // Loop over each block of this tetromino and compare it
        // to the offset within the grid where we want to move to
        for y in 0..4 {
            for x in 0..4 {
                if self.blocks[y][x].is_some() {
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
                    if grid[y as usize][x as usize].is_some() {
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
        let desired_blocks = self.get_blocks_for_direction(new_dir, None);

        // Loop over each block of this tetromino and compare it
        // to the offset within the grid where we want to move to
        for (y, y_block) in desired_blocks.iter().enumerate().take(4) {
            for (x, x_block) in y_block.iter().enumerate().take(4) {
                if x_block.is_some() {
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
                    if grid[y as usize][x as usize].is_some() {
                        // Deny left and right.. but block downwards
                        return RotationResult::Deny;
                    }
                }
            }
        }

        RotationResult::Allow
    }

    /// Tests a cloned Tetromino to find where this current Tetromino
    /// will land
    pub fn find_landing_xy(&self, grid: &[[Option<Block>; 10]; 22]) -> (i32, i32) {
        let mut clone = *self;

        loop {
            if let MoveResult::Allow = clone.can_move(Direction::South, grid) {
                clone.drop_down();
            } else {
                return (clone.x, clone.y);
            }
        }
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
        self.blocks = self.get_blocks_for_direction(new_dir, None);
        self.shadow = self.get_blocks_for_direction(new_dir, true);
        self.direction = new_dir;
    }

    fn get_blocks_for_direction<O>(&self,
                                   direction: Direction,
                                   shadow: O)
                                   -> [[Option<Block>; 4]; 4]
        where O: Into<Option<bool>>
    {
        let s = shadow.into();
        if s.is_some() && s.unwrap() {
            match direction {
                Direction::North => self.shadow_north,
                Direction::East => self.shadow_east,
                Direction::South => self.shadow_south,
                Direction::West => self.shadow_west,

            }
        } else {
            match direction {
                Direction::North => self.north,
                Direction::East => self.east,
                Direction::South => self.south,
                Direction::West => self.west,
            }
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
        self.y += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }
}

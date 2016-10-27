
use game::block::Block;
use game::config::Config;
use game::tetromino::*;

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
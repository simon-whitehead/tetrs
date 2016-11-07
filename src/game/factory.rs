use game::rand::{thread_rng, Rng};

use game::block::Block;
use game::config::Config;
use game::tetromino::*;

pub struct TetrominoFactory {
}

impl TetrominoFactory {
    pub fn new() -> TetrominoFactory {
        TetrominoFactory {}
    }

    pub fn create(&self, config: &Config) -> Tetromino {
        let mut functions: Vec<fn(f32) -> TetrominoShape> = Vec::new();
        functions.push(Self::create_I);
        functions.push(Self::create_J);
        functions.push(Self::create_L);
        functions.push(Self::create_O);
        functions.push(Self::create_S);
        functions.push(Self::create_T);
        functions.push(Self::create_Z);

        let random_number = thread_rng().gen_range(0, 6);

        let shape = (functions[random_number as usize])(1.0);
        let shadow = (functions[random_number as usize])(0.65);

        let shape = TetrominoShape(shape.0, shape.1, shape.2, shape.3);
        let shadow = TetrominoShape(shadow.0, shadow.1, shadow.2, shadow.3);

        Tetromino::new(shape, shadow, &config)
    }

    fn create_blended_color(color: [f32; 3], opacity: f32) -> [f32; 4] {
        // If we're opaque, just return it
        if opacity >= 1.0 {
            [color[0], color[1], color[2], 1.0]
        } else {
            // Otherwise, blend it in to black
            let r = color[0] * (1.0 - opacity);
            let g = color[1] * (1.0 - opacity);
            let b = color[2] * (1.0 - opacity);

            [r, g, b, 1.0]
        }
    }

    fn create_I(opacity: f32) -> TetrominoShape {
        let color = [0.0, 1.0, 1.0];
        let i_color = Self::create_blended_color(color, opacity);
        let i = Some(Block::new(i_color));

        TetrominoShape(// North
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
                        [None, i.clone(), None, None]])
    }

    fn create_J(opacity: f32) -> TetrominoShape {
        let color = [0.0, 0.0, 1.0];
        let j_color = Self::create_blended_color(color, opacity);
        let j = Some(Block::new(j_color));

        TetrominoShape(// North
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
                        [None, None, None, None]])
    }

    fn create_L(opacity: f32) -> TetrominoShape {
        let color = [0.8, 0.5, 0.0];
        let l_color = Self::create_blended_color(color, opacity);
        let l = Some(Block::new(l_color));

        TetrominoShape(// North
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
                        [None, None, None, None]])
    }

    fn create_O(opacity: f32) -> TetrominoShape {
        let color = [1.0, 1.0, 0.0];
        let o_color = Self::create_blended_color(color, opacity);
        let o = Some(Block::new(o_color));

        TetrominoShape(// North
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
                        [None, None, None, None]])
    }

    fn create_S(opacity: f32) -> TetrominoShape {
        let color = [0.0, 0.75, 0.0];
        let s_color = Self::create_blended_color(color, opacity);
        let s = Some(Block::new(s_color));

        TetrominoShape(// North
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
                        [None, None, None, None]])
    }

    fn create_T(opacity: f32) -> TetrominoShape {
        let color = [1.0, 0.4, 0.7];
        let t_color = Self::create_blended_color(color, opacity);
        let t = Some(Block::new(t_color));

        TetrominoShape(// North
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
                        [None, None, None, None]])
    }

    fn create_Z(opacity: f32) -> TetrominoShape {
        let color = [1.0, 0.0, 0.0];
        let z_color = Self::create_blended_color(color, opacity);
        let z = Some(Block::new(z_color));

        TetrominoShape(// North
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
                        [None, None, None, None]])
    }
}

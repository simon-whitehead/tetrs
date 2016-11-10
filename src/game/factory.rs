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
        functions.push(Self::create_i);
        functions.push(Self::create_j);
        functions.push(Self::create_l);
        functions.push(Self::create_o);
        functions.push(Self::create_s);
        functions.push(Self::create_t);
        functions.push(Self::create_z);

        let random_number = thread_rng().gen_range(0, 7);

        let shape = (functions[random_number as usize])(1.0);
        let shadow = (functions[random_number as usize])(0.65);

        let shape = TetrominoShape(shape.0, shape.1, shape.2, shape.3);
        let shadow = TetrominoShape(shadow.0, shadow.1, shadow.2, shadow.3);

        Tetromino::new(shape, shadow, config)
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

    fn create_i(opacity: f32) -> TetrominoShape {
        let color = [0.0, 1.0, 1.0];
        let i_color = Self::create_blended_color(color, opacity);
        let i = Some(Block::new(i_color));

        TetrominoShape(// North
                       [[None, None, None, None],
                        [i, i, i, i],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // East
                       [[None, None, i, None],
                        [None, None, i, None],
                        [None, None, i, None],
                        [None, None, i, None]],

                       // South
                       [[None, None, None, None],
                        [None, None, None, None],
                        [i, i, i, i],
                        [None, None, None, None]],

                       // West
                       [[None, i, None, None],
                        [None, i, None, None],
                        [None, i, None, None],
                        [None, i, None, None]])
    }

    fn create_j(opacity: f32) -> TetrominoShape {
        let color = [0.0, 0.0, 1.0];
        let j_color = Self::create_blended_color(color, opacity);
        let j = Some(Block::new(j_color));

        TetrominoShape(// North
                       [[j, None, None, None],
                        [j, j, j, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // East
                       [[None, j, j, None],
                        [None, j, None, None],
                        [None, j, None, None],
                        [None, None, None, None]],

                       // South
                       [[None, None, None, None],
                        [j, j, j, None],
                        [None, None, j, None],
                        [None, None, None, None]],

                       // West
                       [[None, j, None, None],
                        [None, j, None, None],
                        [j, j, None, None],
                        [None, None, None, None]])
    }

    fn create_l(opacity: f32) -> TetrominoShape {
        let color = [0.8, 0.5, 0.0];
        let l_color = Self::create_blended_color(color, opacity);
        let l = Some(Block::new(l_color));

        TetrominoShape(// North
                       [[None, None, l, None],
                        [l, l, l, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // East
                       [[None, l, None, None],
                        [None, l, None, None],
                        [None, l, l, None],
                        [None, None, None, None]],

                       // South
                       [[None, None, None, None],
                        [l, l, l, None],
                        [l, None, None, None],
                        [None, None, None, None]],

                       // West
                       [[l, l, None, None],
                        [None, l, None, None],
                        [None, l, None, None],
                        [None, None, None, None]])
    }

    fn create_o(opacity: f32) -> TetrominoShape {
        let color = [1.0, 1.0, 0.0];
        let o_color = Self::create_blended_color(color, opacity);
        let o = Some(Block::new(o_color));

        TetrominoShape(// North
                       [[None, o, o, None],
                        [None, o, o, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // East
                       [[None, o, o, None],
                        [None, o, o, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // South
                       [[None, o, o, None],
                        [None, o, o, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // West
                       [[None, o, o, None],
                        [None, o, o, None],
                        [None, None, None, None],
                        [None, None, None, None]])
    }

    fn create_s(opacity: f32) -> TetrominoShape {
        let color = [0.0, 0.75, 0.0];
        let s_color = Self::create_blended_color(color, opacity);
        let s = Some(Block::new(s_color));

        TetrominoShape(// North
                       [[None, s, s, None],
                        [s, s, None, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // East
                       [[None, s, None, None],
                        [None, s, s, None],
                        [None, None, s, None],
                        [None, None, None, None]],

                       // South
                       [[None, None, None, None],
                        [None, s, s, None],
                        [s, s, None, None],
                        [None, None, None, None]],

                       // West
                       [[s, None, None, None],
                        [s, s, None, None],
                        [None, s, None, None],
                        [None, None, None, None]])
    }

    fn create_t(opacity: f32) -> TetrominoShape {
        let color = [1.0, 0.4, 0.7];
        let t_color = Self::create_blended_color(color, opacity);
        let t = Some(Block::new(t_color));

        TetrominoShape(// North
                       [[None, t, None, None],
                        [t, t, t, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // East
                       [[None, t, None, None],
                        [None, t, t, None],
                        [None, t, None, None],
                        [None, None, None, None]],

                       // South
                       [[None, None, None, None],
                        [t, t, t, None],
                        [None, t, None, None],
                        [None, None, None, None]],

                       // West
                       [[None, t, None, None],
                        [t, t, None, None],
                        [None, t, None, None],
                        [None, None, None, None]])
    }

    fn create_z(opacity: f32) -> TetrominoShape {
        let color = [1.0, 0.0, 0.0];
        let z_color = Self::create_blended_color(color, opacity);
        let z = Some(Block::new(z_color));

        TetrominoShape(// North
                       [[z, z, None, None],
                        [None, z, z, None],
                        [None, None, None, None],
                        [None, None, None, None]],

                       // East
                       [[None, None, None, z],
                        [None, None, z, z],
                        [None, None, z, None],
                        [None, None, None, None]],

                       // South
                       [[None, None, None, None],
                        [z, z, None, None],
                        [None, z, z, None],
                        [None, None, None, None]],

                       // West
                       [[None, z, None, None],
                        [z, z, None, None],
                        [z, None, None, None],
                        [None, None, None, None]])
    }
}

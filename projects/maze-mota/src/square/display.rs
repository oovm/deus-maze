use crate::square::{Joint, Maze2D};
use ndarray::Array2;
use std::fmt::{Debug, Display, Formatter};

impl Debug for Joint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Joint({}, {}, {:?})", self.x, self.y, self.direction)
    }
}

impl Display for Maze2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let matrix = self.matrix01();
        let (width, height) = matrix.dim();
        for y in 0..height {
            for x in 0..width {
                if matrix[[x, y]] {
                    write!(f, "■")?;
                }
                else {
                    write!(f, "□")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Maze2D {
    // ╔╤╗
    // ╟ ╢
    // ╚╧╝
    pub fn get_box_character(&self, x: usize, y: usize, m01: &Array2<bool>) -> char {
        let (w, h) = m01.dim();
        let mut c = '■';
        if x > 0 && m01[[x - 1, y]] {
            c = '╟';
        }
        else if x < w - 1 && m01[[x + 1, y]] {
            c = '╢';
        }
        else if y > 0 && m01[[x, y - 1]] {
            c = '╧';
        }
        else if y < h - 1 && m01[[x, y + 1]] {
            c = '╤';
        }
        else if x > 0 && y > 0 && m01[[x - 1, y - 1]] {
            c = '╚';
        }
        else if x < w - 1 && y > 0 && m01[[x + 1, y - 1]] {
            c = '╝';
        }
        else if x > 0 && y < h - 1 && m01[[x - 1, y + 1]] {
            c = '╔';
        }
        else if x < w - 1 && y < h - 1 && m01[[x + 1, y + 1]] {
            c = '╗';
        }
        c
    }

    pub fn render_box_drawings(&self) -> String {
        let matrix = self.matrix01();
        let (width, height) = matrix.dim();
        let mut s = String::new();
        for y in 0..height {
            for x in 0..width {
                if matrix[[x, y]] {
                    s.push(self.get_box_character(x, y, &matrix));
                }
                else {
                    s.push('□');
                }
            }
            s.push('\n');
        }
        s
    }
}

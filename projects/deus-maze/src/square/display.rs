use crate::square::{Joint, Maze2D};
use std::fmt::{Debug, Display, Formatter};

impl Debug for Joint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Joint({}, {}, {:?})", self.x, self.y, self.direction)
    }
}

impl Display for Maze2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let matrix = self.render();
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

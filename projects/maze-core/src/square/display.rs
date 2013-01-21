use crate::square::{Joint, Maze2D};
use std::fmt::{Debug, Display, Formatter};

impl Debug for Joint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Joint({}, {}, {:?})", self.x, self.y, self.direction)
    }
}

impl Display for Maze2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let matrix = self.matrix01();
        for line in matrix.rows(false) {
            for (_, _, point) in line {
                if *point {
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

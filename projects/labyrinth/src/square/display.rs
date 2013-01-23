use crate::square::Maze2D;
use std::fmt::{Debug, Display, Formatter};

impl Debug for Maze2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Maze2D")
            .field("width", &self.config.width)
            .field("height", &self.config.height)
            .field("joints", &self.joints)
            .finish()
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

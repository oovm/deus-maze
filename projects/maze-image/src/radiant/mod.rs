pub struct MazeRadiantRenderer {
    block_size: usize,
}

impl Default for MazeRadiantRenderer {
    fn default() -> Self {
        Self { block_size: 10 }
    }
}

impl MazeRadiantRenderer {
    pub fn new(size: usize) -> Self {
        Self { block_size: size, ..Default::default() }
    }
}

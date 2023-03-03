use crate::types;

#[derive(Debug)]
pub struct Signals {
    size_x: types::Dimension,
    size_y: types::Dimension,
    level: Vec<u8>,
}

impl Signals {
    pub fn new(sx: u16, sy: u16) -> Self {
        Self {
            size_x: sx,
            size_y: sy,
            level: Vec::<u8>::with_capacity((sx * sy).into()),
        }
    }

    pub fn zero_fill(&mut self) {
        for i in 0..self.level.len() {
            self.level[i] = 0;
        }
    }

    pub fn level(&self, ix: u16, iy: u16) -> u8 {
        self.level[(ix * self.size_y + iy) as usize]
    }
}

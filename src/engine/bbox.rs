//! Implementing a bounding box
use crate::engine::Coordinate;

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub far_top: usize,
    pub far_left: usize,
    pub far_right: usize,
    pub far_bottom: usize,
}

impl BoundingBox {
    pub fn increase_y(&mut self, amount: usize) {
        self.far_top += amount;
        self.far_bottom += amount;
    }

    pub fn decrease_y(&mut self, amount: usize) {
        self.far_top -= amount;
        self.far_bottom -= amount;
    }

    pub fn increase_x(&mut self, amount: usize) {
        self.far_right += amount;
        self.far_left += amount;
    }

    pub fn decrease_x(&mut self, amount: usize) {
        self.far_right -= amount;
        self.far_left -= amount;
    }
}

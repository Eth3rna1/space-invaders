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
    pub fn increase_y(&mut self) {
        self.far_top += 1;
        self.far_bottom += 1;
    }

    pub fn decrease_y(&mut self) {
        self.far_top -= 1;
        self.far_bottom -= 1;
    }

    pub fn increase_x(&mut self) {
        self.far_right += 1;
        self.far_left += 1;
    }

    pub fn decrease_x(&mut self) {
        self.far_right -= 1;
        self.far_left -= 1;
    }
}

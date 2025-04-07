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

impl<const S: usize> From<&[Coordinate; S]> for BoundingBox {
    fn from(arr: &[Coordinate; S]) -> Self {
        let mut far_top: usize = arr[0].1;
        let mut far_left: usize = arr[0].0;
        let mut far_right: usize = arr[0].0;
        let mut far_bottom: usize = arr[0].1;
        for coor in arr {
            if coor.1 > far_bottom {
                far_bottom = coor.1;
            }
            if coor.0 > far_right {
                far_right = coor.0;
            }
            if coor.1 < far_top {
                far_top = coor.1;
            }
            if coor.0 < far_left {
                far_left = coor.0;
            }
        }
        Self {
            far_top,
            far_left,
            far_right,
            far_bottom,
        }
    }
}

impl<const S: usize> From<[Coordinate; S]> for BoundingBox {
    fn from(arr: [Coordinate; S]) -> Self {
        let mut far_top: usize = arr[0].1;
        let mut far_left: usize = arr[0].0;
        let mut far_right: usize = arr[0].0;
        let mut far_bottom: usize = arr[0].1;
        for coor in arr {
            if coor.1 > far_bottom {
                far_bottom = coor.1;
            }
            if coor.0 > far_right {
                far_right = coor.0;
            }
            if coor.1 < far_top {
                far_top = coor.1;
            }
            if coor.0 < far_left {
                far_left = coor.0;
            }
        }
        Self {
            far_top,
            far_left,
            far_right,
            far_bottom,
        }
    }
}

impl From<Vec<Coordinate>> for BoundingBox {
    fn from(arr: Vec<Coordinate>) -> Self {
        let mut far_top: usize = arr[0].1;
        let mut far_left: usize = arr[0].0;
        let mut far_right: usize = arr[0].0;
        let mut far_bottom: usize = arr[0].1;
        for coor in arr.iter() {
            if coor.1 > far_bottom {
                far_bottom = coor.1;
            }
            if coor.0 > far_right {
                far_right = coor.0;
            }
            if coor.1 < far_top {
                far_top = coor.1;
            }
            if coor.0 < far_left {
                far_left = coor.0;
            }
        }
        Self {
            far_top,
            far_left,
            far_right,
            far_bottom,
        }
    }
}

impl From<&Vec<Coordinate>> for BoundingBox {
    fn from(arr: &Vec<Coordinate>) -> Self {
        let mut far_top: usize = arr[0].1;
        let mut far_left: usize = arr[0].0;
        let mut far_right: usize = arr[0].0;
        let mut far_bottom: usize = arr[0].1;
        for coor in arr.iter() {
            if coor.1 > far_bottom {
                far_bottom = coor.1;
            }
            if coor.0 > far_right {
                far_right = coor.0;
            }
            if coor.1 < far_top {
                far_top = coor.1;
            }
            if coor.0 < far_left {
                far_left = coor.0;
            }
        }
        Self {
            far_top,
            far_left,
            far_right,
            far_bottom,
        }
    }
}

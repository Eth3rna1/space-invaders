//! Axis-Aligned Bounding Box Utility
//!
//! This module provides a `BoundingBox` struct used to calculate and manage rectangular
//! boundaries around a collection of 2D coordinates. It is primarily intended for use
//! in sprite boundary detection and constraint checking within an ASCII-based rendering
//! engine or game environment.
//!
//! # Purpose
//! The `BoundingBox` tracks the extremities (`top`, `left`, `right`, and `bottom`) of a
//! set of points. It is commonly used to determine whether a sprite is within bounds
//! or has collided with other areas of the screen or grid.
//!
//! # Features
//! - Construct a bounding box from arrays, slices, or vectors of coordinates
//! - Modify position using directional shift methods (`increase_x`, `decrease_y`, etc.)
//! - Copy, clone, and debug-print capabilities
//!
//! # Usage Example
//! ```rust
//! use crate::engine::bounding_box::BoundingBox;
//! use crate::engine::Coordinate;
//!
//! let points: Vec<Coordinate> = vec![(1, 2), (4, 5), (2, 3)];
//! let bbox = BoundingBox::from(&points);
//! assert_eq!(bbox.far_left, 1);
//! assert_eq!(bbox.far_right, 4);
//! assert_eq!(bbox.far_top, 2);
//! assert_eq!(bbox.far_bottom, 5);
//! ```
//!
//! Note: There is no `BoundingBox::new()` constructor by design.
//! Use the `From` trait implementations for initialization.
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

// There is no ::new() initializer function.
// For readability, all initializations for
// `BoundingBox` are done through the From trait.

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

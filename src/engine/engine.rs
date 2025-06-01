//! ASCII Engine Renderer
//!
//! This module provides a customizable ASCII-based rendering engine. It simulates a
//! pixel grid using a matrix of `PixelState` values, where each "pixel" can be either
//! `On` or `Off`. The engine is designed to support rendering operations, simple sprite
//! positioning, and pixel-level manipulation, making it suitable for terminal-based
//! games, visualization tools, or educational graphics demos.
//!
//! # Features
//! - Fixed-size grid engine with toggleable pixel states
//! - Simple collision toggle system
//! - Support for both `Rc<RefCell<...>>` and `Arc<RwLock<...>>` wrapped references for flexible ownership models
//! - ASCII display output for terminal rendering
//! - Matrix reset, clear, and pixel swapping functionalities
//!
//! # Example
//! ```
//! use crate::engine::Engine;
//!
//! let mut engine = Engine::new((10, 5));
//! engine.spawn((2, 3));
//! println!("{}", engine.display('#', '.'));
//! ```
//!
//! This will render a 10x5 grid with a single `#` character at position (2, 3).
use crate::engine::bounding_box::BoundingBox;
use crate::engine::sprite;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelState {
    On,
    Off,
}

/// The engine responsible for rendering the plane
#[derive(Debug, Clone)]
pub struct Engine {
    /// if false, sprites will overlap each other
    collisions: bool,
    /// height of the plane
    pub height: usize,
    /// width of the plane
    pub width: usize,
    /// The matrix representing the plane
    matrix: Vec<Vec<PixelState>>,
}

impl Engine {
    /// initializer function
    ///
    /// takes in a tuple of `(usize, usize)` integers.
    /// The tuple translates to (width, height).
    pub fn new(dimensions: (usize, usize)) -> Self {
        let (width, height) = dimensions;
        let matrix: Vec<Vec<PixelState>> = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| PixelState::Off)
                    .collect::<Vec<PixelState>>()
            })
            .collect();
        Self {
            collisions: true,
            height,
            width,
            matrix,
        }
    }

    /// Setter function modying the collisions internal variable
    /// `collisions` is by default true. This function must be used
    /// if the user decides to not want collisions otherwise.
    pub fn set_collisions(mut self, v: bool) -> Self {
        self.collisions = v;
        self
    }

    /// Getter function returning the state of the `collision` variable
    pub fn collisions(&self) -> bool {
        self.collisions
    }

    /// Returns self as a Reference Counted pointer for shared access
    pub fn as_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    /// Returns self as an Atomic Reference Counted pointer for thread shared access
    pub fn as_arc(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }

    /// Given a coordinate, this function returns the state of the "pixel"
    pub fn is_on(&self, coordinate: &Coordinate) -> bool {
        self.matrix[coordinate.1][coordinate.0] == PixelState::On
    }

    /// Given a coordinate, this function returns the state of the "pixel"
    pub fn is_off(&self, coordinate: &Coordinate) -> bool {
        self.matrix[coordinate.1][coordinate.0] == PixelState::Off
    }

    /// Given a coordinate, this functions sets the state
    /// of a "pixel" to being on.
    pub fn spawn(&mut self, coordinate: Coordinate) {
        self.matrix[coordinate.1][coordinate.0] = PixelState::On;
    }

    /// Swaps the pixel value of two coordinates
    pub fn swap(&mut self, c1: Coordinate, c2: Coordinate) {
        let tmp = self.matrix[c1.1][c1.0];
        self.matrix[c1.1][c1.0] = self.matrix[c2.1][c2.0];
        self.matrix[c2.1][c2.0] = tmp;
    }

    /// Returns a visual representation of the underlying matrix
    pub fn display(&self, pixel_char: char, background_char: char) -> String {
        let mut interface = String::new();
        for r in 0..self.height {
            for c in 0..self.width {
                match self.matrix[r][c] {
                    PixelState::On => interface.push(pixel_char),
                    PixelState::Off => interface.push(background_char),
                }
            }
            interface += "\n";
        }
        interface
    }

    /// Given a coordinate, sets the state of a
    /// "pixel" to being off.
    pub fn reset(&mut self, pixel: &Coordinate) {
        let (x, y) = *pixel;
        self.matrix[y][x] = PixelState::Off;
    }

    /// All "pixels" states are set to off
    pub fn clear(&mut self) {
        self.matrix = (0..self.height)
            .map(|_| {
                (0..self.width)
                    .map(|_| PixelState::Off)
                    .collect::<Vec<PixelState>>()
            })
            .collect();
    }
}

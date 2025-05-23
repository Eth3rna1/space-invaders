/*
    PixelState::Off represents a background character
    PixelState::On represents a pixel char
*/
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

#[derive(Debug, Clone)]
pub struct Engine {
    collisions: bool,
    pub height: usize,
    pub width: usize,
    matrix: Vec<Vec<PixelState>>,
}

impl Engine {
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

    pub fn set_collisions(mut self, v: bool) -> Self {
        self.collisions = v;
        self
    }

    pub fn collisions(&self) -> bool {
        self.collisions
    }

    pub fn as_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn as_arc(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }

    pub fn is_on(&self, coordinate: &Coordinate) -> bool {
        self.matrix[coordinate.1][coordinate.0] == PixelState::On
    }

    pub fn is_off(&self, coordinate: &Coordinate) -> bool {
        self.matrix[coordinate.1][coordinate.0] == PixelState::Off
    }

    pub fn spawn(&mut self, coordinate: Coordinate) {
        self.matrix[coordinate.1][coordinate.0] = PixelState::On;
    }

    pub fn swap(&mut self, c1: Coordinate, c2: Coordinate) {
        let tmp = self.matrix[c1.1][c1.0];
        self.matrix[c1.1][c1.0] = self.matrix[c2.1][c2.0];
        self.matrix[c2.1][c2.0] = tmp;
    }

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

    pub fn reset(&mut self, pixel: &Coordinate) {
        let (x, y) = *pixel;
        self.matrix[y][x] = PixelState::Off;
    }

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

/*
    PixelState::Off represents a background character
    PixelState::On represents a pixel char
*/
pub mod sprite;
//pub mod art;
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};

use std::cell::RefCell;
use std::rc::Rc;

pub type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelState {
    On,
    Off,
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub length: usize,
    pub width: usize,
    matrix: Vec<Vec<PixelState>>,
    pub collisions: bool,
}

impl Engine {
    pub fn new(dimensions: (usize, usize)) -> Self {
        let (width, length) = dimensions;
        let matrix: Vec<Vec<PixelState>> = (0..length)
            .map(|_| {
                (0..width)
                    .map(|_| PixelState::Off)
                    .collect::<Vec<PixelState>>()
            })
            .collect();
        Self {
            length,
            width,
            matrix,
            collisions: true,
        }
    }

    pub fn as_rc(self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }

    pub fn is_on(&self, coordinate: Coordinate) -> bool {
        self.matrix[coordinate.1][coordinate.0] == PixelState::On
    }

    pub fn is_off(&self, coordinate: Coordinate) -> bool {
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

    pub fn output(&self) -> String {
        let mut interface = String::new();
        for r in 0..self.length {
            for c in 0..self.width {
                match self.matrix[r][c] {
                    PixelState::On => interface.push(PIXEL_CHAR),
                    PixelState::Off => interface.push(BACKGROUND_CHAR),
                }
            }
            interface += "\n";
        }
        interface
    }

    pub fn reset(&mut self, pixel: Coordinate) {
        let (x, y) = pixel;
        self.matrix[y][x] = PixelState::Off;
    }

    pub fn clear(&mut self) {
        self.matrix = (0..self.length)
            .map(|_| {
                (0..self.width)
                    .map(|_| PixelState::Off)
                    .collect::<Vec<PixelState>>()
            })
            .collect();
    }
}

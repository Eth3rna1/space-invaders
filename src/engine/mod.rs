/*
    Status::Off represents a background character
    Status::On represents a pixel char
*/
pub mod sprite;
pub mod art;
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};


pub type Coordinate = (usize, usize);


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    On,
    Off
}

pub struct Engine {
    pub length: usize,
    pub width: usize,
    matrix: Vec<Vec<Status>>,
}


impl Engine {
    pub fn new(dimensions: (usize, usize)) -> Self {
        let (width, length) = dimensions;
        let mut matrix: Vec<Vec<Status>> = Vec::new();
        (0..length)
            .for_each(|_| matrix.push((0..width).map(|_| Status::Off).collect::<Vec<Status>>()));
        Self {
            length,
            width,
            matrix,
        }
    }

    pub fn is_on(&self, coordinate : Coordinate) -> bool {
        self.matrix[coordinate.1][coordinate.0] == Status::On
    }

    pub fn is_off(&self, coordinate : Coordinate) -> bool {
        self.matrix[coordinate.1][coordinate.0] == Status::Off
    }


    pub fn spawn(&mut self, coordinate: Coordinate) {
        self.matrix[coordinate.1][coordinate.0] = Status::On;
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
                    Status::On  => interface.push(PIXEL_CHAR),
                    Status::Off => interface.push(BACKGROUND_CHAR)
                }
            }
            interface += "\n";
        }
        interface
    }


    pub fn reset(&mut self, pixel : Coordinate) {
        let (x, y) = pixel;
        self.matrix[y][x] = Status::Off;
    }
}

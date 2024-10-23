pub mod pool;
pub mod art;
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};

pub struct Engine {
    pub length: usize,
    pub width: usize,
    matrix: Vec<Vec<char>>,
}


impl Engine {
    pub fn new(dimensions: (usize, usize)) -> Self {
        let (width, length) = dimensions;
        let mut matrix: Vec<Vec<char>> = Vec::new();
        (0..length)
            .for_each(|_| matrix.push((0..width).map(|_| BACKGROUND_CHAR).collect::<Vec<char>>()));
        Self {
            length,
            width,
            matrix,
        }
    }


    pub fn populated(&self, coordinate : (usize, usize)) -> bool {
        return self.matrix[coordinate.1][coordinate.0] == BACKGROUND_CHAR
    }


    pub fn spawn(&mut self, c: char, coordinate: (usize, usize)) {
        self.matrix[coordinate.1][coordinate.0] = c;
    }


    pub fn swap(&mut self, c1: (usize, usize), c2: (usize, usize)) {
        let tmp = self.matrix[c1.1][c1.0];
        self.matrix[c1.1][c1.0] = self.matrix[c2.1][c2.0];
        self.matrix[c2.1][c2.0] = tmp;
    }


    pub fn output(&self) -> String {
        let mut interface = String::new();
        for r in 0..self.length {
            for c in 0..self.width {
                interface.push(self.matrix[r][c]);
            }
            interface.push('\n');
        }
        interface
    }


    pub fn reset(&mut self, pixel : (usize, usize)) {
        let (x, y) = pixel;
        self.matrix[y][x] = BACKGROUND_CHAR;
    }
}

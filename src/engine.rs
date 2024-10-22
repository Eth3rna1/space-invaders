/*
    Ascii renderer
*/
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
}

pub fn spawn_pool(engine: &mut Engine, pool: &[(usize, usize)]) {
    for coordinate in pool {
        engine.spawn(PIXEL_CHAR, *coordinate);
    }
}

pub fn move_pool_right(engine: &mut Engine, pool: &mut [(usize, usize)]) {
    for i in 0..pool.len() {
        let new = (pool[i].0 + 1, pool[i].1);
        engine.swap(pool[i], new);
        pool[i] = new;
    }
}

pub fn move_pool_left(engine: &mut Engine, pool: &mut Vec<(usize, usize)>) {
    todo!()
}

pub fn move_pool_up(engine: &mut Engine, pool: &mut Vec<(usize, usize)>) {
    todo!()
}

pub fn move_pool_down(engine: &mut Engine, pool: &mut Vec<(usize, usize)>) {
    todo!()
}

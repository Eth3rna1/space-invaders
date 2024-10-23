/*
    Ascii renderer
*/
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};
use crate::tool;
use crate::render::Render;

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

pub mod pool {
    use super::{
        Engine,
        BACKGROUND_CHAR,
        PIXEL_CHAR
    };

    pub fn spawn_pool(engine: &mut Engine, pool: &[(usize, usize)]) {
        for coordinate in pool {
            engine.spawn(PIXEL_CHAR, *coordinate);
        }
    }

    pub fn move_pool_right(engine: &mut Engine, pool: &mut [(usize, usize)]) {
        for coor in pool {
            let new = (coor.0 + 1, coor.1);
            engine.swap(*coor, new);
            *coor = new;
        }
    }

    pub fn move_pool_left(engine: &mut Engine, pool: &mut [(usize, usize)]) {
        for coor in pool {
            let new = (coor.0 - 1, coor.1);
            engine.swap(*coor, new);
            *coor = new;
        }
    }

    pub fn move_pool_up(engine: &mut Engine, pool: &mut [(usize, usize)]) {
        for coor in pool {
            let new = (coor.0, coor.1 - 1);
            engine.swap(*coor, new);
            *coor = new;
        }
    }

    pub fn move_pool_down(engine: &mut Engine, pool: &mut [(usize, usize)]) {
        for coor in pool {
            let new = (coor.0, coor.1 + 1);
            engine.swap(*coor, new);
            *coor = new;
        }
    }
}

/// Returns a pool of coordinates
fn draw_line(engine : &mut Engine, _start : (usize, usize), _end : (usize, usize)) -> Result<Vec<(usize, usize)>, String> {
    // I am not understanding the concept
    // I need to draw paper examples
    //    Todo: find the next coordinate point between a start and end point
    //
    //     Matrix: (+ start)  (- end)
    //     |---------------------------|
    //     |                           |
    //     |    +?                     |
    //     |      ?                    |
    //     |       ?                   |
    //     |        ?????              |
    //     |             ?             |
    //     |              ? ??         |
    //     |                  ? -      |
    //     |                           |
    //     |                           |
    //     |---------------------------|
    //
    //
    //     Answer:
    //         define the left coordinate and right coordinate from start and end
    //         and get the slope
    //
    //     Formula:     
    //             rise      y2 - y1
    //            ------  = --------- = m (slope)
    //              run      x2 - x2
    todo!()
}

use crate::engine::Engine;
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};


pub fn spawn_pool(engine: &mut Engine, pool: &[(usize, usize)]) {
    for coordinate in pool {
        engine.spawn(*coordinate);
    }
}

pub fn move_pool_right(engine: &mut Engine, pool: &mut [(usize, usize)]) {
    // reversed the array
    for coor in pool.into_iter().rev() {
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
    // reversed the array
    for coor in pool.into_iter().rev() {
        let new = (coor.0, coor.1 + 1);
        engine.swap(*coor, new);
        *coor = new;
    }
}



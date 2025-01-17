use crate::engine::Engine;
use crate::engine::Coordinate;
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};
use crate::errors::{Error, ErrorKind};


pub struct Sprite<'a> {
    engine : &'a Engine,
    coordinates : &'a [Coordinate]
}

impl<'a> Sprite<'a> {
    pub fn new(engine : &'a Engine, coordinates : &'a [Coordinate]) -> Self {
        Self {
            engine,
            coordinates
        }
    }

    pub fn exists(&'a self) -> bool {
        self.coordinates.iter().all(|coor| self.engine.is_on(*coor))
    }

    pub fn move_down(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn move_up(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn move_right(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn move_left(&mut self) -> Result<(), Error> {
        todo!()
    }
}


pub fn spawn_sprite(engine: &mut Engine, sprite: &[Coordinate]) {
    for coordinate in sprite {
        engine.spawn(*coordinate);
    }
}

pub fn move_sprite_right(engine: &mut Engine, sprite: &mut [Coordinate]) {
    // reversed the array
    for coor in sprite.into_iter().rev() {
        let new = (coor.0 + 1, coor.1);
        engine.swap(*coor, new);
        *coor = new;
    }
}

pub fn move_sprite_left(engine: &mut Engine, sprite: &mut [Coordinate]) {
    for coor in sprite {
        let new = (coor.0 - 1, coor.1);
        engine.swap(*coor, new);
        *coor = new;
    }
}

pub fn move_sprite_up(engine: &mut Engine, sprite: &mut [Coordinate]) {
    for coor in sprite {
        let new = (coor.0, coor.1 - 1);
        engine.swap(*coor, new);
        *coor = new;
    }
}

pub fn move_sprite_down(engine: &mut Engine, sprite: &mut [Coordinate]) {
    // reversed the array
    for coor in sprite.into_iter().rev() {
        let new = (coor.0, coor.1 + 1);
        engine.swap(*coor, new);
        *coor = new;
    }
}



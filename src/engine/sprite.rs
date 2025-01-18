use crate::engine::Engine;
use crate::engine::Coordinate;
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};
use crate::errors::{Error, ErrorKind};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Sprite {
    engine : Rc<RefCell<Engine>>,
    coordinates : Rc<RefCell<Vec<Coordinate>>>
}

impl Sprite {
    pub fn new<const SIZE: usize>(engine : Rc<RefCell<Engine>>, coordinates : [Coordinate; SIZE]) -> Result<Self, Error> {
        {
            // checking that all coordinates
            // are within boundaries
            for coor in coordinates.iter() {
                if coor.0 >= 0 && coor.1 < engine.borrow().length {
                    continue
                }
                return Err(Error::new(ErrorKind::OutOfBounds, format!(r#"Coordinates don't fit boundaries
Referenced coordinate: {:?}

Engine width and length: ({}, {})
"#, coor, engine.borrow().width, engine.borrow().length)));
            }
        }
        Ok(Self {
            engine,
            coordinates : Rc::new(RefCell::new(coordinates.to_vec()))
        })
    }

    pub fn exists(&self) -> bool {
        self.coordinates.borrow().iter().all(|coor| self.engine.borrow().is_on(*coor))
    }

    pub fn spawn(&mut self) {
        todo!();
    }

    pub fn move_down(&mut self) -> Result<(), Error> {
        // since the array is reversed, I have to index into the first item
        // to assert that the sprite stays within boundaries
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



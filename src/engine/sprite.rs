use crate::engine::Engine;
use crate::engine::Coordinate;
use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};
use crate::errors::{Error, ErrorKind};

use std::rc::Rc;
use std::cell::RefCell;


fn _sort_coordinates(coordinates : &mut [Coordinate]) {
    // Sorting by x first, then by y
    coordinates.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1) // Compare x-coordinates if y-coordinates are equal
        } else {
            a.0.cmp(&b.0) // Compare y-coordinates if different
        }
    });
}


#[derive(Debug, Clone)]
pub struct Sprite {
    engine      : Rc<RefCell<Engine>>,
    coordinates : Vec<Coordinate>,
    far_right   : usize,
    far_left    : usize,
    far_top     : usize,
    far_bottom  : usize
}

impl Sprite {
    pub fn new(engine : Rc<RefCell<Engine>>, coordinates : Vec<Coordinate>) -> Result<Self, Error> {
        if coordinates.len() == 0 {
            return Err(Error::new(ErrorKind::InexistentSprite, "Not enough coordinates to create a sprite"));
        }
        let mut coordinates = coordinates;
        _sort_coordinates(&mut coordinates);
        {
            let eng = engine.borrow();
            // checking that all coordinates
            // fit within the engine boundaries
            if !coordinates.iter().all(|coor| {
                coor.0 >= 0 && coor.1 < eng.length
            }) {
                return Err(Error::new(ErrorKind::OutOfBounds, format!("Coordinates do not fit within ({}, {})", eng.width, eng.length)));
            }
        }
        let c = &coordinates[0];
        let far_top: usize = c.1;
        let far_left: usize = c.0;
        let mut far_right: usize = c.0;
        let mut far_bottom: usize = c.1;
        match coordinates.len() {
            //0 => already got covered in the first assertion
            1 => {} // already got covered
            _ => {
                // far_left and far_top already got covered so no need to reassign
                for i in 1..coordinates.len() {
                    let c = &coordinates[i];
                    if c.0 > far_right {
                        far_right = c.0;
                    }
                    if c.1 > far_bottom {
                        far_bottom = c.1;
                    }
                }
            }
        }
        Ok(Self {
            engine,
            coordinates,
            far_right,
            far_left,
            far_top,
            far_bottom
        })
    }
    
    pub fn spawn(&mut self) {
        let mut engine = self.engine.borrow_mut();
        spawn_sprite(&mut engine, &mut self.coordinates);
        drop(engine);
    }

    pub fn move_up(&mut self) -> Result<(), Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // checking that the sprite stays within the engine's boundaries
            if self.far_top as isize - 1 < 0 {
                return Err(Error::new(ErrorKind::OutOfBounds, format!("Y value does not fit within the engine boundaries;

Far Top Y-Value: {}

Engine's Dimensions: ({}, {})", self.far_top, engine.width, engine.length)));
            }
        }
        move_sprite_down(&mut engine, &mut self.coordinates);
        drop(engine);
        self.far_top -= 1;
        self.far_bottom -= 1;
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), Error> {
        let mut engine = self.engine.borrow_mut();
        {
            if self.far_left as isize - 1 < 0 {
                return Err(Error::new(ErrorKind::OutOfBounds, format!("Y value does not fit within the engine boundaries;

Far Left X-Value: {:?}

Engine's Dimensions: ({}, {})", self.far_left, engine.width, engine.length)));
            }
        }
        {
            move_sprite_left(&mut engine, &mut self.coordinates);
        }
        drop(engine);
        self.far_left -= 1;
        self.far_right -= 1;
        Ok(())
    }

    pub fn move_right(&mut self) -> Result<(), Error> {
        // reminder that the array gets reversed
        let mut engine = self.engine.borrow_mut();
        {
            if self.far_right + 1 >= engine.width {
                return Err(Error::new(ErrorKind::OutOfBounds, format!("Y value does not fit within the engine boundaries;

Far Right X-Value: {:?}

Engine's Dimensions: ({}, {})", self.far_right, engine.width, engine.length)));
            }
        }
        {
            move_sprite_right(&mut engine, &mut self.coordinates);
        }
        drop(engine);
        self.far_right += 1;
        self.far_left += 1;
        Ok(())
    }

    pub fn move_down(&mut self) -> Result<(), Error> {
        // reminder that array gets reversed
        // assert the first element
        let mut engine = self.engine.borrow_mut();
        {
            if self.far_bottom + 1 >= engine.length {
                return Err(Error::new(ErrorKind::OutOfBounds, format!("Y value does not fit within the engine boundaries;

Referenced Coordinate: {:?}

Engine's Dimensions: ({}, {})", self.far_bottom, engine.width, engine.length)));
            }
        }
        {
            move_sprite_down(&mut engine, &mut self.coordinates);
        }
        drop(engine);
        self.far_bottom += 1;
        self.far_top += 1;
        Ok(())
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



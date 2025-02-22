use crate::engine::bbox::BoundingBox;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::engine::Within;
use crate::errors::{Error, ErrorKind};
use crate::{BACKGROUND_CHAR, PIXEL_CHAR};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

fn _sort_coordinates(coordinates: &mut [Coordinate]) {
    // Sorting by x first, then by y
    coordinates.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1) // Compare x-coordinates if y-coordinates are equal
        } else {
            a.0.cmp(&b.0) // Compare y-coordinates if different
        }
    });
}

/// State of the sprite after a method has been called
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Collided,
    Destroyed,
    Spawned,
    Moved,
}

/// A group of pixels
#[derive(Debug, Clone)]
pub struct Sprite {
    collisions: bool,
    engine: Rc<RefCell<Engine>>,
    coordinates: Vec<Coordinate>,
    velocity: usize,
    pub(crate) bounding_box: BoundingBox,
}

/* /// THIS CODE IS GOING TO BE IMPLEMENTED LATER IN THE NEW FUNCTION
fn update_boundaries(&mut self) {
} */

impl Sprite {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        coordinates: Vec<Coordinate>,
        velocity: usize,
    ) -> Result<Self, Error> {
        {
            //error cases
            if coordinates.is_empty() {
                return Err(Error::new(
                    ErrorKind::InexistentSprite,
                    "Not enough coordinates to create a sprite",
                ));
            }
            let __eng = engine.borrow();
            // checking that all coordinates
            // fit within the engine boundaries
            if !coordinates
                .iter()
                .all(|coor| coor.0 < __eng.width && coor.1 < __eng.length)
            {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!(
                        "Coordinates do not fit within ({}, {})",
                        __eng.width, __eng.length
                    ),
                ));
            }
            if coordinates.iter().any(|coor| __eng.is_on(*coor)) {
                return Err(Error::new(
                    ErrorKind::OverlappingSprite,
                    "A sprite already exists within given coordinates",
                ));
            }
        }
        let mut coordinates = coordinates;
        _sort_coordinates(&mut coordinates);
        let mut far_top: usize = coordinates[0].1;
        let mut far_left: usize = coordinates[0].0;
        let mut far_right: usize = coordinates[0].0;
        let mut far_bottom: usize = coordinates[0].1;

        for coor in coordinates.iter() {
            if coor.1 > far_bottom {
                far_bottom = coor.1;
            }
            if coor.0 > far_right {
                far_right = coor.0;
            }
        }
        let bounding_box = BoundingBox {
            far_top,
            far_left,
            far_right,
            far_bottom,
        };
        Ok(Self {
            collisions: true,
            engine,
            velocity,
            coordinates,
            bounding_box,
        })
    }

    /// Checks for collisions with other pixels whose pixel state is on
    pub fn set_collisions(mut self, boolean: bool) -> Self {
        self.collisions = boolean;
        self
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        //if coordinate.within(self.bounding_box) {
        //    true
        //}
        //false
        coordinate.within(&self.bounding_box)
    }

    pub fn pop(&mut self, coordinate: Coordinate) -> Result<(), Error> {
        if !self.contains(coordinate) {
            return Err(Error::new(ErrorKind::InexistentCoordinate, format!("Cannot pop coordinate because it doesn't exist within `{:?}`, referenced coordinate: {:?}", self as *const Sprite, coordinate)));
        }
        let index = {
            let mut idx = 0;
            for (i, coor) in self.coordinates.iter().enumerate() {
                //idx += 1;
                if *coor == coordinate {
                    idx = i;
                    break;
                }
            }
            idx
        };
        self.coordinates.remove(index);
        Ok(())
    }

    pub fn spawn(&mut self) -> State {
        let mut engine = self.engine.borrow_mut();
        {
            spawn_sprite(&mut engine, &mut self.coordinates);
        }
        State::Spawned
    }

    pub fn move_up(&mut self) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // checking that the sprite stays within the engine's boundaries
            //if self.far_top.1 as isize - 1 < 0 {
            // no need to convert to a signed integer
            if self.bounding_box.far_top == 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "Hit the far top boundry",
                ));
            }
        }
        {
            for i in 0..self.velocity {
                {
                    if self.bounding_box.far_top == 0 {
                        return Err(Error::new(
                            ErrorKind::OutOfBounds,
                            "Hit the far top boundry",
                        ));
                    }
                }
                {
                    // collision detection
                    if engine.is_on((self.bounding_box.far_left, self.bounding_box.far_top - 1))
                        && self.collisions
                    {
                        return Ok(State::Collided);
                    }
                }
                {
                    move_sprite_down(&mut engine, &mut self.coordinates);
                    self.bounding_box.decrease_y(1);
                }
            }
        }
        Ok(State::Moved)
    }

    pub fn move_left(&mut self) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // error case
            //if self.far_left.0 as isize - 1 < 0 {
            // no need to convert to a signed integer
            if self.bounding_box.far_left == 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "Hit the far left boundry",
                ));
            }
        }
        {
            for i in 0..self.velocity {
                {
                    if self.bounding_box.far_left == 0 {
                        return Err(Error::new(
                            ErrorKind::OutOfBounds,
                            "Hit the far left boundry",
                        ));
                    }
                }
                {
                    if engine.is_on((self.bounding_box.far_left - 1, self.bounding_box.far_top))
                        && self.collisions
                    {
                        return Ok(State::Collided);
                    }
                }
                {
                    move_sprite_left(&mut engine, &mut self.coordinates);
                    self.bounding_box.decrease_x(1);
                }
            }
        }
        Ok(State::Moved)
    }

    pub fn move_right(&mut self) -> Result<State, Error> {
        // reminder that the array gets reversed
        let mut engine = self.engine.borrow_mut();
        {
            // error case
            if self.bounding_box.far_right == engine.width - 1 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "Hit the far right boundry",
                ));
            }
        }
        {
            for i in 0..self.velocity {
                {
                    if self.bounding_box.far_right == engine.width - 1 {
                        return Err(Error::new(
                            ErrorKind::OutOfBounds,
                            "Hit the far right boundry",
                        ));
                    }
                }
                {
                    if engine.is_on((
                        self.bounding_box.far_right + 1,
                        self.bounding_box.far_bottom,
                    )) && self.collisions
                    {
                        return Ok(State::Collided);
                    }
                }
                {
                    move_sprite_right(&mut engine, &mut self.coordinates);
                    self.bounding_box.increase_x(1);
                }
            }
        }
        Ok(State::Moved)
    }

    pub fn move_down(&mut self) -> Result<State, Error> {
        // reminder that array gets reversed
        // assert the first element
        let mut engine = self.engine.borrow_mut();
        {
            // error case
            if self.bounding_box.far_bottom == engine.length - 1 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "hit the far bottom boundry",
                ));
            }
        }
        {
            for i in 0..self.velocity {
                {
                    if self.bounding_box.far_bottom == engine.length - 1 {
                        return Err(Error::new(
                            ErrorKind::OutOfBounds,
                            "Hit the far bottom boundry",
                        ));
                    }
                }
                {
                    if engine.is_on((self.bounding_box.far_left, self.bounding_box.far_bottom + 1))
                        && self.collisions
                    {
                        return Ok(State::Collided);
                    }
                }
            }
        }
        Ok(State::Moved)
    }

    pub fn destroy(&mut self) -> State {
        let mut engine = self.engine.borrow_mut();
        for coor in self.coordinates.iter() {
            engine.reset(*coor);
        }
        State::Destroyed
    }
}

pub fn spawn_sprite(engine: &mut Engine, sprite: &[Coordinate]) {
    for coordinate in sprite {
        engine.spawn(*coordinate);
    }
}

pub fn move_sprite_right(engine: &mut Engine, sprite: &mut [Coordinate]) {
    // reversed the array
    for coor in sprite.iter_mut().rev() {
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
    for coor in sprite.iter_mut().rev() {
        let new = (coor.0, coor.1 + 1);
        engine.swap(*coor, new);
        *coor = new;
    }
}

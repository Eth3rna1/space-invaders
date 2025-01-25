use crate::constants::{BACKGROUND_CHAR, PIXEL_CHAR};
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

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
    far_top: Coordinate,
    far_left: Coordinate,
    far_right: Coordinate,
    far_bottom: Coordinate,
    far_top_coordinates: Vec<Coordinate>,
    far_left_coordinates: Vec<Coordinate>,
    far_right_coordinates: Vec<Coordinate>,
    far_bottom_coordinates: Vec<Coordinate>,
}

/* /// THIS CODE IS GOING TO BE IMPLEMENTED LATER IN THE NEW FUNCTION
fn update_boundaries(&mut self) {
    let mut far_top = self.coordinates[0];
    let mut far_bottom = self.coordinates[0];
    let mut far_left = self.coordinates[0];
    let mut far_right = self.coordinates[0];

    let mut far_top_coordinates = vec![];
    let mut far_bottom_coordinates = vec![];
    let mut far_left_coordinates = vec![];
    let mut far_right_coordinates = vec![];

    for &coor in &self.coordinates {
        // Update bounds
        if coor.1 < far_top.1 {
            far_top = coor;
            far_top_coordinates.clear();
        }
        if coor.1 == far_top.1 {
            far_top_coordinates.push(coor);
        }

        if coor.1 > far_bottom.1 {
            far_bottom = coor;
            far_bottom_coordinates.clear();
        }
        if coor.1 == far_bottom.1 {

        }

        if coor.0 < far_left.0 {
            far_left = coor;
            far_left_coordinates.clear();
        }
        if coor.0 == far_left.0 {
            far_left_coordinates.push(coor);
        }

        if coor.0 > far_right.0 {
            far_right = coor;
            far_right_coordinates.clear();
        }
        if coor.0 == far_right.0 {
            far_right_coordinates.push(coor);
        }
    }

    // Assign new values
    self.far_top = far_top;
    self.far_bottom = far_bottom;
    self.far_left = far_left;
    self.far_right = far_right;
    self.far_top_coordinates = far_top_coordinates;
    self.far_bottom_coordinates = far_bottom_coordinates;
    self.far_left_coordinates = far_left_coordinates;
    self.far_right_coordinates = far_right_coordinates;
} */

impl Sprite {
    pub fn new(engine: Rc<RefCell<Engine>>, coordinates: Vec<Coordinate>) -> Result<Self, Error> {
        let collisions = true;
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
        //let c = coordinates[0];
        let far_top: Coordinate = *coordinates.iter().min_by_key(|coor| coor.1).unwrap();
        let far_left: Coordinate = *coordinates.iter().min_by_key(|coor| coor.0).unwrap();
        let far_right: Coordinate = *coordinates.iter().max_by_key(|coor| coor.0).unwrap();
        let far_bottom: Coordinate = *coordinates.iter().max_by_key(|coor| coor.1).unwrap();
        let far_top_coordinates: Vec<Coordinate> = coordinates
            .iter()
            .filter(|coor| coor.1 == far_top.1)
            .cloned()
            .collect();
        let far_left_coordinates: Vec<Coordinate> = coordinates
            .iter()
            .filter(|coor| coor.0 == far_left.0)
            .cloned()
            .collect();
        let far_right_coordinates: Vec<Coordinate> = coordinates
            .iter()
            .filter(|coor| coor.0 == far_right.0)
            .cloned()
            .collect();
        let far_bottom_coordinates: Vec<Coordinate> = coordinates
            .iter()
            .filter(|coor| coor.1 == far_bottom.1)
            .cloned()
            .collect();
        Ok(Self {
            collisions,
            engine,
            coordinates,
            far_top,
            far_left,
            far_right,
            far_bottom,
            far_top_coordinates,
            far_left_coordinates,
            far_right_coordinates,
            far_bottom_coordinates,
        })
    }

    /// Checks for collisions with other pixels whose pixel state is on
    pub fn set_collisions(mut self, boolean: bool) -> Self {
        self.collisions = boolean;
        self
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        if self.far_left.0 <= coordinate.0 && coordinate.0 <= self.far_right.0
        && self.far_top.1 <= coordinate.1 && coordinate.1 <= self.far_bottom.1 {
            return true
        }
        false
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
            if self.far_top.1 == 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!(
                        "Y value does not fit within the engine boundaries;

Far Top Y-Value: {}

Engine's Dimensions: ({}, {})",
                        self.far_top.1, engine.width, engine.length
                    ),
                ));
            }
        }
        {
            // checking for collisions
            if self.far_top_coordinates.iter().any(|coor| {
                let upcoming_coor: Coordinate = (coor.0, coor.1 - 1);
                engine.is_on(upcoming_coor)
            }) && self.collisions
            {
                return Ok(State::Collided);
            }
        }
        {
            move_sprite_up(&mut engine, &mut self.coordinates);
        }
        self.far_top.1 -= 1;
        self.far_bottom.1 -= 1;
        self.far_top_coordinates = self
            .far_top_coordinates
            .iter()
            .map(|coor| (coor.0, coor.1 - 1))
            .collect();
        Ok(State::Moved)
    }

    pub fn move_left(&mut self) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // error case
            //if self.far_left.0 as isize - 1 < 0 {
            // no need to convert to a signed integer
            if self.far_left.0 == 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!(
                        "Y value does not fit within the engine boundaries;

Far Left X-Value: {:?}

Engine's Dimensions: ({}, {})",
                        self.far_left.0, engine.width, engine.length
                    ),
                ));
            }
        }
        {
            // checking for collisions
            if self.far_top_coordinates.iter().any(|coor| {
                let upcoming_coor: Coordinate = (coor.0 - 1, coor.1);
                engine.is_on(upcoming_coor)
            }) && self.collisions
            {
                return Ok(State::Collided);
            }
        }
        {
            move_sprite_left(&mut engine, &mut self.coordinates);
        }
        self.far_left.0 -= 1;
        self.far_right.0 -= 1;
        self.far_left_coordinates = self
            .far_left_coordinates
            .iter()
            .map(|coor| (coor.0 - 1, coor.1))
            .collect();
        Ok(State::Moved)
    }

    pub fn move_right(&mut self) -> Result<State, Error> {
        // reminder that the array gets reversed
        let mut engine = self.engine.borrow_mut();
        {
            // error case
            if self.far_right.0 + 1 >= engine.width {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!(
                        "Y value does not fit within the engine boundaries;

Far Right X-Value: {:?}

Engine's Dimensions: ({}, {})",
                        self.far_right.0, engine.width, engine.length
                    ),
                ));
            }
        }
        {
            // checking for collisions
            if self.far_top_coordinates.iter().any(|coor| {
                let upcoming_coor: Coordinate = (coor.0 + 1, coor.1);
                engine.is_on(upcoming_coor)
            }) && self.collisions
            {
                return Ok(State::Collided);
            }
        }
        {
            move_sprite_right(&mut engine, &mut self.coordinates);
        }
        self.far_right.0 += 1;
        self.far_left.0 += 1;
        self.far_right_coordinates = self
            .far_right_coordinates
            .iter()
            .map(|coor| (coor.0 + 1, coor.1))
            .collect();
        Ok(State::Moved)
    }

    pub fn move_down(&mut self) -> Result<State, Error> {
        // reminder that array gets reversed
        // assert the first element
        let mut engine = self.engine.borrow_mut();
        {
            // error case
            if self.far_bottom.1 + 1 >= engine.length {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!(
                        "Y value does not fit within the engine boundaries;

Referenced Coordinate: {:?}

Engine's Dimensions: ({}, {})",
                        self.far_bottom.1, engine.width, engine.length
                    ),
                ));
            }
        }
        {
            // checking for collisions
            if self.far_top_coordinates.iter().any(|coor| {
                let upcoming_coor: Coordinate = (coor.0, coor.1 + 1);
                engine.is_on(upcoming_coor)
            }) && self.collisions
            {
                return Ok(State::Collided);
            }
        }
        {
            move_sprite_down(&mut engine, &mut self.coordinates);
        }
        self.far_bottom.1 += 1;
        self.far_top.1 += 1;
        self.far_bottom_coordinates = self
            .far_bottom_coordinates
            .iter()
            .map(|coor| (coor.0, coor.1 + 1))
            .collect();
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

use crate::engine::bounding_box::BoundingBox;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

/// State of the sprite after a method has been called
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum State {
    Collided(Coordinate),
    Destroyed,
    Spawned,
    Moved,
    Hit,
    Null,
}

/// A group of pixels
#[derive(Debug, Clone)]
pub struct Sprite {
    pub(crate) engine: Rc<RefCell<Engine>>,
    pub(crate) coordinates: Vec<Coordinate>,
    // one velocity for both axises
    pub(crate) velocity: f64,
    pub(crate) bounding_box: BoundingBox,
    is_spawned: bool,
    is_destroyed: bool,
    /// The pin point floating number X position of the sprite
    exact_x: f64,
    /// The pin point floating number Y position of the sprite
    exact_y: f64,
}

impl Sprite {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        coordinates: Vec<Coordinate>,
        velocity: f64,
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
                .all(|coor| coor.0 < __eng.width && coor.1 < __eng.height)
            {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!(
                        "Coordinates do not fit within ({}, {})",
                        __eng.width, __eng.height
                    ),
                ));
            }
        }
        let bounding_box = BoundingBox::from(&coordinates);
        Ok(Self {
            engine,
            velocity,
            coordinates,
            bounding_box,
            is_spawned: false,
            is_destroyed: false,
            exact_x: bounding_box.far_left as f64,
            exact_y: bounding_box.far_top as f64,
        })
    }

    pub fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    pub fn is_spawned(&self) -> bool {
        self.is_spawned
    }

    pub fn far_top(&self) -> usize {
        self.bounding_box.far_top
    }

    pub fn far_left(&self) -> usize {
        self.bounding_box.far_left
    }

    pub fn far_right(&self) -> usize {
        self.bounding_box.far_right
    }

    pub fn far_bottom(&self) -> usize {
        self.bounding_box.far_bottom
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.coordinates.contains(&coordinate)
    }

    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    pub fn exact_x(&self) -> f64 {
        self.exact_x
    }

    pub fn offset_exact_x(&mut self, offset: f64) {
        self.exact_x += offset;
    }

    pub fn exact_y(&self) -> f64 {
        self.exact_y
    }

    pub fn offset_exact_y(&mut self, offset: f64) {
        self.exact_y += offset;
    }

    /// A function that looks into the future next left position
    pub fn next_step_left(&self, delta_time: f64) -> usize {
        (self.exact_x - self.velocity * delta_time) as usize
    }

    /// A function that looks into the future next right position
    pub fn next_step_right(&self, delta_time: f64) -> usize {
        (self.exact_x + self.velocity * delta_time) as usize
    }

    pub fn next_step_up(&self, delta_time: f64) -> usize {
        (self.exact_y - self.velocity * delta_time) as usize
    }

    pub fn next_step_down(&self, delta_time: f64) -> usize {
        (self.exact_y + self.velocity * delta_time) as usize
    }

    pub fn pop(&mut self, coordinate: Coordinate) -> Result<State, Error> {
        if self.is_destroyed {
            return Ok(State::Destroyed);
        }
        if !self.contains(coordinate) {
            return Err(Error::new(ErrorKind::InexistentCoordinate, format!("Cannot pop coordinate because it doesn't exist within `{:?}`, referenced coordinate: {:?}", self as *const Sprite, coordinate)));
        }
        let index = {
            let mut idx = 0;
            for (i, coor) in self.coordinates.iter().enumerate() {
                if *coor == coordinate {
                    idx = i;
                    break;
                }
            }
            idx
        };
        self.coordinates.remove(index);
        Ok(State::Hit)
    }

    pub fn spawn(&mut self) -> Result<State, Error> {
        self.is_destroyed = false;
        self.is_spawned = true;
        let mut eng = self.engine.borrow_mut();
        {
            if self.coordinates.iter().any(|coor| eng.is_on(coor)) {
                return Err(Error::new(
                    ErrorKind::OverlappingSprite,
                    "A sprite already exists within given coordinates",
                ));
            }
        }
        {
            for coordinate in self.coordinates.iter() {
                eng.spawn(*coordinate);
            }
        }
        Ok(State::Spawned)
    }

    pub fn move_up(&mut self, delta_time: f64) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // error checking if the sprite is already touching the boundry
            if self.bounding_box.far_top == 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "Hit the far right boundry",
                ));
            }
        }
        // calculating the EXACT floating point offset number. In other words, the EXACT step;
        // the code following the `offset` variable, is going to work with rounding
        // such number
        let offset: f64 = self.velocity * delta_time;
        // obtaining the difference between the new X position and the current X position
        let step: usize = (self.exact_y + offset) as usize - self.exact_y as usize;
        self.exact_y -= offset;
        if step == 0 {
            // this if statement
            // reduces redundancy
            // if the step is 0
            // that means there
            // is no visual momentum
            // and thus, there is no
            // need to update anything
            return Ok(State::Null);
        }
        // checking if the step leads the sprite out of boundries, if so, a new step is assigned
        let step = if self.bounding_box.far_top as i32 - step as i32 <= 0 {
            // reassigning to its starting position
            let new_step = self.bounding_box.far_top;
            self.exact_y = self.bounding_box.far_top as f64;
            //self.exact_y -= self.exact_y; // 0
            new_step
        } else {
            step
        };
        if engine.collisions() {
            // collision detection
            for col in self.bounding_box.far_right..=self.bounding_box.far_left {
                let future_coordinate = (col, self.bounding_box.far_top - step);
                if engine.is_on(&future_coordinate) {
                    self.exact_y += offset; // reseting the offset
                    return Ok(State::Collided(future_coordinate));
                }
            }
        }
        if self.is_spawned {
            // reseting the current position
            for coordinate in self.coordinates.iter() {
                engine.reset(coordinate);
            }
        }
        {
            // drawing or assigning the new position
            for coordinate in self.coordinates.iter_mut() {
                let new = (coordinate.0, coordinate.1 - step);
                if self.is_spawned {
                    engine.spawn(new);
                }
                *coordinate = new;
            }
            self.bounding_box.decrease_y(step);
        }
        Ok(State::Moved)
    }

    pub fn move_left(&mut self, delta_time: f64) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // error checking if the sprite is already touching the boundry
            if self.bounding_box.far_left == 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "Hit the far right boundry",
                ));
            }
        }
        // calculating the EXACT floating point offset number. In other words, the EXACT step;
        // the code following the `offset` variable, is going to work with rounding
        // such number
        let offset: f64 = self.velocity * delta_time;
        // obtaining the difference between the new X position and the current X position
        let step: usize = (self.exact_x + offset) as usize - self.exact_x as usize;
        self.exact_x -= offset;
        if step == 0 {
            // this if statement
            // reduces redundancy
            // if the step is 0
            // that means there
            // is no visual momentum
            // and thus, there is no
            // need to update anything
            return Ok(State::Null);
        }
        // checking if the step leads the sprite out of boundries, if so, a new step is assigned
        let step = if self.bounding_box.far_left as i32 - step as i32 <= 0 {
            //              |
            //              |
            //              |
            //  (0th index) 0            #
            //              |            ^
            //              |          Pixel within the X axis
            //              |
            //              -------------------------------
            //              0    1   2   3
            //
            // Assigning the new step with the current X position
            // leads to this equation:
            //
            //          0 = x - x
            //
            //  x is the number that represents the sprites
            //  position within the X axis
            //
            //  Essentially, since the sprite is destined to go out of boundries
            //  I want to at least make the X position be 0. How am I going to do that?
            //  As shown by the equation, subtracting the X position by itself, the
            //  value will cancel out and will equal 0.
            let new_step = self.bounding_box.far_left;
            self.exact_x = self.bounding_box.far_left as f64;
            //self.exact_x -= self.exact_x; // 0
            new_step
        } else {
            step
        };
        if engine.collisions() {
            // collision detection
            for row in self.bounding_box.far_top..=self.bounding_box.far_bottom {
                let future_coordinate = (self.bounding_box.far_left - step, row);
                if engine.is_on(&future_coordinate) {
                    self.exact_x += offset; // reseting the offset
                    return Ok(State::Collided(future_coordinate));
                }
            }
        }
        if self.is_spawned {
            // reseting the current position
            for coordinate in self.coordinates.iter() {
                engine.reset(coordinate);
            }
        }
        {
            // drawing the new position
            for coordinate in self.coordinates.iter_mut() {
                let new = (coordinate.0 - step, coordinate.1);
                if self.is_spawned {
                    engine.spawn(new);
                }
                *coordinate = new;
            }
            self.bounding_box.decrease_x(step);
        }
        Ok(State::Moved)
    }

    pub fn move_right(&mut self, delta_time: f64) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // error checking if the sprite is already touching the boundry
            if self.bounding_box.far_right == engine.width - 1 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "Hit the far right boundry",
                ));
            }
        }
        // calculating the EXACT floating point offset number. In other words, the EXACT step;
        // the code following the `offset` variable, is going to work with rounding
        // such number
        let offset: f64 = self.velocity * delta_time;
        // obtaining the difference between the new X position and the current X position
        let step: usize = (self.exact_x + offset) as usize - self.exact_x as usize;
        self.exact_x += offset;
        if step == 0 {
            // this if statement
            // reduces redundancy
            // if the step is 0
            // that means there
            // is no visual momentum
            // and thus, there is no
            // need to update anything
            return Ok(State::Null);
        }
        // checking if the step leads the sprite out of boundries, if so, a new step is assigned
        let step = if self.bounding_box.far_right + step >= engine.width {
            let new_step = engine.width - self.bounding_box.far_right - 1;
            // + new step because the coordinates haven't been updated yet
            //self.exact_x = (self.bounding_box.far_left + new_step) as f64;
            self.exact_x = (self.bounding_box.far_right + new_step) as f64;
            new_step
        } else {
            step
        };
        if engine.collisions() {
            // collision detection; looking into the future step if it is populated
            for row in self.bounding_box.far_top..=self.bounding_box.far_bottom {
                let future_coordinate = (self.bounding_box.far_right + step, row);
                if engine.is_on(&future_coordinate) {
                    self.exact_x -= offset; // reseting the offset
                    return Ok(State::Collided(future_coordinate));
                }
            }
        }
        if self.is_spawned {
            // reseting the current position
            for coordinate in self.coordinates.iter() {
                engine.reset(coordinate);
            }
        }
        {
            // drawing the new position
            for coordinate in self.coordinates.iter_mut() {
                let new = (coordinate.0 + step, coordinate.1);
                if self.is_spawned {
                    engine.spawn(new);
                }
                *coordinate = new;
            }
            self.bounding_box.increase_x(step);
        }
        Ok(State::Moved)
    }

    pub fn move_down(&mut self, delta_time: f64) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        {
            // error checking if the sprite is already touching the boundry
            if self.bounding_box.far_bottom == engine.height - 1 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    "Hit the far bottom boundry",
                ));
            }
        }
        // calculating the EXACT floating point offset number. In other words, the EXACT step;
        // the code following the `offset` variable, is going to work with rounding
        // such number
        let offset: f64 = self.velocity * delta_time;
        // obtaining the difference between the new X position and the current X position
        let step: usize = (self.exact_y + offset) as usize - self.exact_y as usize;
        self.exact_y += offset;
        if step == 0 {
            // this if statement
            // reduces redundancy
            // if the step is 0
            // that means there
            // is no visual momentum
            // and thus, there is no
            // need to update anything
            return Ok(State::Null);
        }
        // checking if the step leads the sprite out of boundries, if so, a new step is assigned
        let step = if self.bounding_box.far_bottom + step >= engine.height {
            let new_step = engine.height - self.bounding_box.far_bottom - 1;
            // + new step because the coordinates haven't been updated yet
            self.exact_y = (self.bounding_box.far_top + new_step) as f64;
            new_step
        } else {
            step
        };
        if engine.collisions() {
            // collision detection
            for col in self.bounding_box.far_right..=self.bounding_box.far_left {
                let future_coordinate = (col, self.bounding_box.far_bottom + step);
                if engine.is_on(&future_coordinate) {
                    self.exact_y += offset; // reseting the offset
                    return Ok(State::Collided(future_coordinate));
                }
            }
        }
        if self.is_spawned {
            // reseting the current position
            for coordinate in self.coordinates.iter() {
                engine.reset(coordinate);
            }
        }
        {
            // drawing the new position
            for coordinate in self.coordinates.iter_mut() {
                let new = (coordinate.0, coordinate.1 + step);
                if self.is_spawned {
                    engine.spawn(new);
                }
                *coordinate = new;
            }
            self.bounding_box.increase_y(step);
        }
        Ok(State::Moved)
    }

    pub fn move_relative_x(&mut self, step: i32) -> Result<State, Error> {
        if step == 0 {
            return Ok(State::Null);
        }
        let mut engine = self.engine.borrow_mut();
        {
            // checking for boundries
            if self.bounding_box.far_left as i32 + step < 0 && step < 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!("Can't move sprite `{:?}` further left", self as *const Self),
                ));
            }
            if self.bounding_box.far_right as i32 + step > (engine.width - 1) as i32 && step > 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!(
                        "Can't move sprite `{:?}` further right",
                        self as *const Self
                    ),
                ));
            }
        }
        if engine.collisions() {
            // checking for collisions
            if step > 0 {
                // positive step, moving right
                for row in self.bounding_box.far_top..=self.bounding_box.far_bottom {
                    let future_coordinate: Coordinate =
                        (self.bounding_box.far_right + step as usize, row);
                    if engine.is_on(&future_coordinate) {
                        return Ok(State::Collided(future_coordinate));
                    }
                }
            } else {
                // negative step, left movement
                for row in self.bounding_box.far_top..=self.bounding_box.far_bottom {
                    let future_coordinate: Coordinate =
                        ((self.bounding_box.far_left as i32 + step) as usize, row);
                    if engine.is_on(&future_coordinate) {
                        return Ok(State::Collided(future_coordinate));
                    }
                }
            }
        }
        if self.is_spawned {
            // reseting the current position
            for coordinate in self.coordinates.iter() {
                engine.reset(coordinate);
            }
        }
        {
            // drawing the new position
            for coordinate in self.coordinates.iter_mut() {
                let new = ((coordinate.0 as i32 + step) as usize, coordinate.1);
                if self.is_spawned {
                    engine.spawn(new);
                }
                *coordinate = new;
            }
            if step > 0 {
                self.bounding_box.increase_x(step as usize);
            } else {
                self.bounding_box.decrease_x(step.abs() as usize);
            }
        }
        Ok(State::Moved)
    }

    pub fn destroy(&mut self) -> State {
        let mut engine = self.engine.borrow_mut();
        for coor in self.coordinates.iter() {
            engine.reset(coor);
        }
        self.is_destroyed = true;
        self.is_spawned = false;
        State::Destroyed
    }
}

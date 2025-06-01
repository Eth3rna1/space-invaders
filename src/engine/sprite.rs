//! Sprite System for ASCII Engine
//!
//! This module provides a `Sprite` abstraction for working with a movable
//! collections of pixels within an ASCII-based engine grid. Sprites are tracked with
//! position, velocity, and bounding box metadata, and are capable of responding to
//! movement, collision, and rendering events.
//!
//! Sprites interact directly with an `Engine` instance, which manages the global
//! rendering matrix. Through this interface, sprites can be spawned, destroyed, or moved
//! in cardinal directions or via arbitrary deltas. The system includes optional collision
//! detection, bounds checking, and state transitions.
//!
//! # Features
//! - Fine-grained pixel group control within the rendering matrix
//! - State-tracked lifecycle: spawning, movement, collision, and destruction
//! - Axis-aligned bounding box tracking for collision and bounds enforcement
//! - Velocity-based movement with time delta input
//! - Coordinate-based removal and runtime updates
//!
//! # Sprite Lifecycle
//! A sprite is constructed with a reference to an `Engine`, a list of coordinates,
//! and initial x/y velocities. It must be explicitly spawned onto the engine to become
//! active. Once spawned, it can be moved or destroyed. Movement functions return
//! a [`State`] indicating the outcome (e.g., moved, collided, etc.).
//!
//! # Example
//! ```rust
//! use crate::engine::{Engine, sprite::Sprite};
//! use std::rc::Rc;
//! use std::cell::RefCell;
//!
//! let engine = Rc::new(RefCell::new(Engine::new((20, 10))));
//! let coordinates = vec![(5, 5), (6, 5)];
//! let mut sprite = Sprite::new(engine.clone(), coordinates, 1.0, 1.0).unwrap();
//!
//! sprite.spawn().unwrap();
//! sprite.move_down(1.0).unwrap();
//! ```
//!
//! # State Handling
//! Every major operation returns a [`State`] enum that indicates the effect of
//! that operation. This includes outcomes such as:
//! - `State::Moved`
//! - `State::Collided((x, y))`
//! - `State::Destroyed`
//! - `State::Null`
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

/// Controls a group of pixels in the engine
#[derive(Debug, Clone)]
pub struct Sprite {
    engine: Rc<RefCell<Engine>>,
    coordinates: Vec<Coordinate>,
    x_velocity: f32,
    y_velocity: f32,
    pub(crate) bounding_box: BoundingBox,
    is_spawned: bool,
    is_destroyed: bool,
    /// delta for horizontal change
    fx: f32,
    /// delta for vertical change
    fy: f32,
}

impl Sprite {
    /// Initializer function.
    ///
    /// Takes in a pointer to the Engine,
    /// the sprites position, horizontal velocity,
    /// and vertical velocity
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        coordinates: Vec<Coordinate>,
        x_velocity: f32,
        y_velocity: f32,
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
            x_velocity,
            y_velocity,
            coordinates,
            bounding_box,
            is_spawned: false,
            is_destroyed: false,
            fx: 0.0,
            fy: 0.0,
        })
    }

    /// Returns a clone of the underlying engine
    pub fn engine(&self) -> Rc<RefCell<Engine>> {
        self.engine.clone()
    }

    /// Returns an array reference to the sprites position
    pub fn coordinates<'c>(&'c self) -> &'c [Coordinate] {
        &self.coordinates
    }

    /// Return a mutable array reference to the sprites position
    pub fn coordinates_mut<'c>(&'c mut self) -> &'c mut [Coordinate] {
        &mut self.coordinates
    }

    /// Returns horizontal position
    pub fn x(&self) -> usize {
        self.bounding_box.far_left
    }

    /// Returns vertical position
    pub fn y(&self) -> usize {
        self.bounding_box.far_top
    }

    /// Returns a coordinate to the sprites position
    pub fn position(&self) -> Coordinate {
        (self.x(), self.y())
    }

    /// Sets a new horizontal velocity
    pub fn set_x_velocity(&mut self, velocity: f32) {
        self.x_velocity = velocity;
    }

    /// Sets a new vertical velocity
    pub fn set_y_velocity(&mut self, velocity: f32) {
        self.y_velocity = velocity;
    }

    /// Returns a clone to the underlying bounding box
    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box.clone()
    }

    /// Returns the state of the underlying is_destroyed boolean
    pub fn is_destroyed(&self) -> bool {
        self.is_destroyed
    }

    /// Returns the state of the underlying is_spawned boolean
    pub fn is_spawned(&self) -> bool {
        self.is_spawned
    }

    /// Returns the recorded far top extreme from the bounding box
    pub fn far_top(&self) -> usize {
        self.bounding_box.far_top
    }

    /// Returns the recorded far top left extreme from the bounding box
    pub fn far_left(&self) -> usize {
        self.bounding_box.far_left
    }

    /// Returns the recorded far top right extreme from the bounding box
    pub fn far_right(&self) -> usize {
        self.bounding_box.far_right
    }

    /// Returns the recorded far bottom right extreme from the bounding box
    pub fn far_bottom(&self) -> usize {
        self.bounding_box.far_bottom
    }

    /// Recalculates the bounding box from the underlying position
    pub fn recalc_bounding_box(&mut self) {
        self.bounding_box = BoundingBox::from(&self.coordinates);
    }

    /// Returns true if a coordinate exists within the position
    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.coordinates.contains(&coordinate)
        // To-Do: calculate via the bounding box, not its position
    }

    /// Getter function: returns horizontal velocity
    pub fn x_velocity(&self) -> f32 {
        self.x_velocity
    }

    /// Getter function: returns vertical velocity
    pub fn y_velocity(&self) -> f32 {
        self.y_velocity
    }

    /// Getter function: returns delta for horizontal change
    pub fn fx(&self) -> f32 {
        self.fx
    }

    /// Getter function: returns delta for horizontal change
    pub fn fy(&self) -> f32 {
        self.fy
    }

    /// turns off a "pixel" within the position of the sprite and removes
    /// it from the body position
    pub fn pop(&mut self, coordinate: Coordinate) -> Result<State, Error> {
        let mut engine = self.engine.borrow_mut();
        if self.is_destroyed {
            return Ok(State::Destroyed);
        }
        for c in 0..self.coordinates.len() {
            if self.coordinates[c] == coordinate {
                engine.reset(&coordinate); // set to off
                let _ = self.coordinates.remove(c);
                return Ok(State::Hit);
            }
        }
        // returns an error by default if the coordinate doesn't exist within the position
        Err(Error::new(ErrorKind::InexistentCoordinate, format!("Cannot pop coordinate because it doesn't exist within `{:?}`, referenced coordinate: {:?}", self as *const Sprite, coordinate)))
    }

    /// Iterates over the position and sets all pixels to true, displaying the sprite
    pub fn spawn(&mut self) -> Result<State, Error> {
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
                eng.spawn(*coordinate); // sets to on
            }
        }
        self.is_destroyed = false;
        self.is_spawned = true;
        Ok(State::Spawned)
    }

    /// Moves the sprite up one time depending on delta time.
    pub fn move_up(&mut self, delta_time: f32) -> Result<State, Error> {
        if !self.is_spawned {
            return Ok(State::Null);
        }
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
        let offset: f32 = self.y_velocity * delta_time;
        let step: usize = {
            // obtaining the difference between the new X position and the current X position,
            // ultimately obtaining a whole number to calculate a step with.
            let calc = (self.fy + offset) as usize - self.fy as usize;
            if calc == 0 {
                // this if statement
                // reduces redundancy
                // if the step is 0
                // that means there
                // is no visual momentum
                // and thus, there is no
                // need to update anything
                self.fy += offset;
                return Ok(State::Null);
            } else if self.bounding_box.far_top as isize - (calc as isize) < 0 {
                self.bounding_box.far_top
            } else {
                calc
            }
        };
        {
            self.fy = 0.0 // reseting vertical delta
        }
        if engine.collisions() {
            // collision detection
            //
            // looks into the future coordinate position and checks
            // if such coordinate is already on.
            for col in self.bounding_box.far_left..=self.bounding_box.far_right {
                let future_coordinate = (col, self.bounding_box.far_top - step);
                if engine.is_on(&future_coordinate) {
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

    pub fn move_left(&mut self, delta_time: f32) -> Result<State, Error> {
        if !self.is_spawned {
            return Ok(State::Null);
        }
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
        let offset: f32 = self.x_velocity * delta_time;
        let step: usize = {
            // obtaining the difference between the new X position and the current X position
            //
            // looks into the future coordinate position and checks
            // if such coordinate is already on.
            let calc = (self.fx + offset) as usize - self.fx as usize;
            if calc == 0 {
                // this if statement
                // reduces redundancy
                // if the step is 0
                // that means there
                // is no visual momentum
                // and thus, there is no
                // need to update anything
                self.fx += offset;
                return Ok(State::Null);
            } else if self.bounding_box.far_left as isize - (calc as isize) < 0 {
                self.bounding_box.far_left
            } else {
                calc
            }
        };
        {
            self.fx = 0.0 // reseting horizontal delta
        }
        if engine.collisions() {
            // collision detection
            //
            // looks into the future coordinate position and checks
            // if such coordinate is already on.
            for row in self.bounding_box.far_top..=self.bounding_box.far_bottom {
                let future_coordinate = (self.bounding_box.far_left - step, row);
                if engine.is_on(&future_coordinate) {
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

    pub fn move_right(&mut self, delta_time: f32) -> Result<State, Error> {
        if !self.is_spawned {
            return Ok(State::Null);
        }
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
        let offset: f32 = self.x_velocity * delta_time;
        let step: usize = {
            // obtaining the difference between the new X position and the current X position
            //
            // looks into the future coordinate position and checks
            // if such coordinate is already on.
            let calc = (self.fx + offset) as usize - self.fx as usize;
            if calc == 0 {
                // this if statement
                // reduces redundancy
                // if the step is 0
                // that means there
                // is no visual momentum
                // and thus, there is no
                // need to update anything
                self.fx += offset;
                return Ok(State::Null);
            } else if self.bounding_box.far_right + calc >= engine.width {
                engine.width - self.bounding_box.far_right - 1
            } else {
                calc
            }
        };
        {
            self.fx = 0.0; // reseting horizontal delta
        }
        if engine.collisions() {
            // collision detection; looking into the future step if it is populated
            for row in self.bounding_box.far_top..=self.bounding_box.far_bottom {
                let future_coordinate = (self.bounding_box.far_right + step, row);
                if engine.is_on(&future_coordinate) {
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

    pub fn move_down(&mut self, delta_time: f32) -> Result<State, Error> {
        if !self.is_spawned {
            return Ok(State::Null);
        }
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
        let offset: f32 = self.y_velocity * delta_time;
        let step: usize = {
            let calc = (self.fy + offset) as usize - self.fy as usize;
            if calc == 0 {
                // this if statement
                // reduces redundancy
                // if the step is 0
                // that means there
                // is no visual momentum
                // and thus, there is no
                // need to update anything
                self.fy += offset;
                return Ok(State::Null);
            } else if self.bounding_box.far_bottom + calc >= engine.height {
                engine.height - self.bounding_box.far_bottom - 1
            } else {
                calc
            }
        };
        {
            self.fy = 0.0 // reseting vertical delta
        }
        if engine.collisions() {
            // collision detection
            for col in self.bounding_box.far_left..=self.bounding_box.far_right {
                let future_coordinate = (col, self.bounding_box.far_bottom + step);
                if engine.is_on(&future_coordinate) {
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

    /// Ignores delta time and delta change variables and forces a step
    /// in the y access.
    ///
    /// right direction = 1 -> inf (positive num)
    /// left direction  = -inf <- -1 (negative num)
    pub fn move_relative_y(&mut self, step: i32) -> Result<State, Error> {
        if !self.is_spawned {
            return Ok(State::Null);
        }
        if step == 0 {
            return Ok(State::Null);
        }
        let mut engine = self.engine.borrow_mut();
        {
            // checking for boundries
            if self.bounding_box.far_top as i32 + step < 0 && step < 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!("Can't move sprite `{:?}` further up", self as *const Self),
                ));
            }
            if self.bounding_box.far_bottom as i32 + step > (engine.height - 1) as i32 && step > 0 {
                return Err(Error::new(
                    ErrorKind::OutOfBounds,
                    format!("Can't move sprite `{:?}` further down", self as *const Self),
                ));
            }
        }
        if engine.collisions() {
            // checking for collisions
            if step > 0 {
                // positive step, moving right
                for col in self.bounding_box.far_left..=self.bounding_box.far_right {
                    let future_coordinate: Coordinate =
                        (col, (self.bounding_box.far_bottom as i32 + step) as usize);
                    if engine.is_on(&future_coordinate) {
                        return Ok(State::Collided(future_coordinate));
                    }
                }
            } else {
                // negative step, left movement
                for col in self.bounding_box.far_left..=self.bounding_box.far_right {
                    let future_coordinate: Coordinate =
                        (col, (self.bounding_box.far_top as i32 + step) as usize);
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
                let new = (coordinate.0, (coordinate.1 as i32 + step) as usize);
                if self.is_spawned {
                    engine.spawn(new);
                }
                *coordinate = new;
            }
            if step > 0 {
                self.bounding_box.increase_y(step as usize);
            } else {
                self.bounding_box.decrease_y(step.abs() as usize);
            }
        }
        Ok(State::Moved)
    }

    /// Ignores delta time and delta change variables and forces a step
    /// in the y access.
    ///
    /// down direction = 1 -> inf (positive num)
    /// up direction   = -inf <- -1 (negative num)
    pub fn move_relative_x(&mut self, step: i32) -> Result<State, Error> {
        if !self.is_spawned {
            return Ok(State::Null);
        }
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

    /// Turns all the pixels under the sprites position to off,
    /// dissapearing from the plane.
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

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
    engine: Rc<RefCell<Engine>>,
    coordinates: Vec<Coordinate>,
    // one velocity for both axises
    velocity: f32,
    pub(crate) bounding_box: BoundingBox,
    is_spawned: bool,
    is_destroyed: bool,
    /// The pin point floating number X position of the sprite
    fx: f32,
    /// The pin point floating number Y position of the sprite
    fy: f32,
}

impl Sprite {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        coordinates: Vec<Coordinate>,
        velocity: f32,
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
            fx: 0.0,
            fy: 0.0,
        })
    }

    pub fn engine(&self) -> Rc<RefCell<Engine>> {
        self.engine.clone()
    }

    pub fn coordinates<'c>(&'c self) -> &'c [Coordinate] {
        &self.coordinates
    }

    pub fn coordinates_mut<'c>(&'c mut self) -> &'c mut [Coordinate] {
        &mut self.coordinates
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box.clone()
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

    pub fn recalc_bounding_box(&mut self) {
        self.bounding_box = BoundingBox::from(&self.coordinates);
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.coordinates.contains(&coordinate)
    }

    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    pub fn fx(&self) -> f32 {
        self.fx
    }

    pub fn fy(&self) -> f32 {
        self.fy
    }

    pub fn pop(&mut self, coordinate: Coordinate) -> Result<State, Error> {
        if self.is_destroyed {
            return Ok(State::Destroyed);
        }
        for c in 0..self.coordinates.len() {
            if self.coordinates[c] == coordinate {
                let _ = self.coordinates.remove(c);
                return Ok(State::Hit);
            }
        }
        Err(Error::new(ErrorKind::InexistentCoordinate, format!("Cannot pop coordinate because it doesn't exist within `{:?}`, referenced coordinate: {:?}", self as *const Sprite, coordinate)))
    }

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
                eng.spawn(*coordinate);
            }
        }
        self.is_destroyed = false;
        self.is_spawned = true;
        Ok(State::Spawned)
    }

    pub fn move_up(&mut self, delta_time: f32) -> Result<State, Error> {
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
        let offset: f32 = self.velocity * delta_time;
        let step: usize = {
            // obtaining the difference between the new X position and the current X position
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
            self.fy = 0.0 // reseting the displacement
        }
        if engine.collisions() {
            // collision detection
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
        let offset: f32 = self.velocity * delta_time;
        let step: usize = {
            // obtaining the difference between the new X position and the current X position
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
            self.fx = 0.0 // reseting the displacement
        }
        if engine.collisions() {
            // collision detection
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
        let offset: f32 = self.velocity * delta_time;
        let step: usize = {
            // obtaining the difference between the new X position and the current X position
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
                //let mut step = 0;
                //for i in self.bounding_box.far_right..engine.width {
                //    if self.bounding_box.far_right + i == engine.width {
                //        step = i;
                //        break;
                //    }
                //}
                //step
            } else {
                calc
            }
        };
        {
            self.fx = 0.0; // reseting the displacement
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
        let offset: f32 = self.velocity * delta_time;
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
                //let mut step = 0;
                //for i in self.bounding_box.far_bottom..engine.height {
                //    if self.bounding_box.far_bottom + i == engine.height {
                //        step = i;
                //        break;
                //    }
                //}
                //step
            } else {
                calc
            }
        };
        {
            self.fy = 0.0 // reseting the displacement
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

    pub fn move_relative_y(&mut self, step: i32) -> Result<State, Error> {
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

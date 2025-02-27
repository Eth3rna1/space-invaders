// The fast paced speedster at the last row
use crate::engine::sprite::{Sprite, State};
use crate::engine::{Coordinate, Engine};
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Speedster {
    original_position: Vec<Coordinate>,
    sprite: Sprite,
}

impl Speedster {
    pub fn new(engine: Rc<RefCell<Engine>>, velocity: usize) -> Result<Self, Error> {
        let position = {
            let eng = engine.borrow();
            vec![(eng.width - 1, 0), (eng.width - 2, 0), (eng.width - 3, 0)]
        };
        Ok(Self {
            original_position: position.clone(),
            sprite: Sprite::new(engine, position, velocity)?,
        })
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.sprite.contains(coordinate)
    }

    pub fn is_destroyed(&self) -> bool {
        self.sprite.is_destroyed()
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.spawn();
    }

    pub fn respawn(&mut self) -> Result<State, Error> {
        // creating a new instance
        // Returns a Result enum, though, nothing no error should occur
        // since the first row is reserved for the speedster
        self.sprite = Sprite::new(
            self.sprite.engine.clone(),
            self.original_position.clone(),
            self.sprite.velocity,
        )?;
        self.spawn();
        Ok(State::Spawned)
    }

    pub fn step(&mut self) -> Result<State, Error> {
        let result = self.sprite.move_left();
        if result.is_err() && result.unwrap_err().kind() == ErrorKind::OutOfBounds {
            self.sprite.destroy();
            return Ok(State::Destroyed);
        }
        Ok(State::Moved)
    }

    pub fn tail(&mut self) -> Coordinate {
        let far_left = self.sprite.bounding_box.far_left;
        let far_right = self.sprite.bounding_box.far_right;
        (
            far_left + ((far_right - far_left) / 2),
            self.sprite.bounding_box.far_bottom + 1, // if doesn't work, try +2
        )
    }

    pub fn destroy(&mut self) -> State {
        self.sprite.destroy()
    }
}

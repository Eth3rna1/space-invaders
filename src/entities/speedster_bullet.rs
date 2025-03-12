/*
    Implementing logic for the Speedsters bullet
*/
use crate::engine::sprite::{Sprite, State};
use crate::engine::{Coordinate, Engine};
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

pub struct SpeedsterBullet {
    sprite: Sprite,
}

impl SpeedsterBullet {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        position: Vec<Coordinate>,
        velocity: f64,
    ) -> Result<Self, Error> {
        Ok(Self {
            sprite: Sprite::new(engine, position, velocity)?,
        })
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.sprite.contains(coordinate)
    }

    /// The speedster bullet is the only sprite that can overlap
    pub fn step(&mut self, delta_time: f64) -> Result<State, Error> {
        let result = self.sprite.move_down(delta_time);
        if let Err(ref error) = result {
            if error.kind() == ErrorKind::OutOfBounds {
                self.sprite.destroy();
                return Ok(State::Null);
            }
        }
        result
    }
}

/*
    Bullet implementation
*/
use crate::engine::sprite::{Sprite, State};
use crate::engine::{Coordinate, Engine};
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Bullet {
    sprite: Sprite,
}

impl Bullet {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        position: Vec<Coordinate>,
        velocity: f64,
    ) -> Result<Self, Error> {
        Ok(Self {
            sprite: Sprite::new(engine, position, velocity)?,
        })
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.spawn();
    }

    pub fn step(&mut self, delta_time: f64) -> Result<State, Error> {
        let result = self.sprite.move_up(delta_time);
        if let Err(ref error) = result {
            if error.kind() == ErrorKind::OutOfBounds {
                self.sprite.destroy();
            }
        }
        result
    }

    pub fn contains(&mut self, coordinate: Coordinate) -> bool {
        self.sprite.contains(coordinate)
    }

    pub fn destroy(&mut self) -> State {
        self.sprite.destroy()
    }
}

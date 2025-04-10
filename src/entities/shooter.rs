use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Shooter {
    sprite: Sprite,
}

impl Shooter {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        position: Vec<Coordinate>,
        velocity: f32,
    ) -> Result<Self, Error> {
        Ok(Self {
            sprite: Sprite::new(engine, position, velocity)?,
        })
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.spawn();
    }

    pub fn step(&mut self, key: &str, delta_time: f32) -> Result<State, Error> {
        match key {
            "left" => self.sprite.move_left(delta_time),
            "right" => self.sprite.move_right(delta_time),
            _ => Ok(State::Null),
        };
        Ok(State::Null)
    }

    pub fn head(&self) -> Coordinate {
        (
            (self.sprite.far_right() - (self.sprite.far_right() - self.sprite.far_left()) / 2),
            self.sprite.far_top() - 1,
        )
    }

    pub fn destroy(&mut self) {
        self.sprite.destroy();
    }
}

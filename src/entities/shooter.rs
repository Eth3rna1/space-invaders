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

    pub fn x(&self) -> usize {
        self.sprite.far_left()
    }

    pub fn xs(&self) -> [usize; 3] {
        let mut x: [usize; 3] = [0, 0, 0];
        let fl = self.sprite.far_left();
        x[0] = fl;
        x[1] = fl - 1;
        x[2] = fl - 2;
        x
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.spawn();
    }

    pub fn contains(&mut self, coordinate: Coordinate) -> bool {
        self.sprite.contains(coordinate)
    }

    pub fn step(&mut self, key: &str, delta_time: f32) -> Option<Coordinate> {
        // moving the sprite
        let result: Result<State, Error> = match key {
            "left" => self.sprite.move_left(delta_time),
            "right" => self.sprite.move_right(delta_time),
            _ => return None,
        };
        // dealing with the movement result
        return match result {
            Ok(state) => match state {
                State::Collided(coordinate) => Some(coordinate),
                _ => None,
            },
            Err(_) => None,
        };
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

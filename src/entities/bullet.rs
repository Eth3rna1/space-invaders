use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::errors::{Error, ErrorKind};
use crate::utils;
use crate::SpaceInvaders;
use crate::BULLET_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Bullet {
    sprite: Sprite,
    is_alien_bullet: bool,
}

impl Bullet {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        position: Coordinate,
        velocity: f32,
    ) -> Result<Self, Error> {
        Ok(Self {
            sprite: Sprite::new(engine, vec![position], velocity)?,
            is_alien_bullet: false,
        })
    }

    pub fn position(&self) -> Coordinate {
        self.sprite.coordinates[0]
    }

    pub fn far_top(&self) -> usize {
        self.sprite.far_top()
    }

    pub fn spawn(&mut self) -> Result<(), Error> {
        let _ = self.sprite.spawn()?;
        Ok(())
    }

    pub fn is_spawned(&self) -> bool {
        self.sprite.is_spawned()
    }

    pub fn to_alien_bullet(mut self) -> Self {
        self.is_alien_bullet = true;
        self
    }

    pub fn is_alien_bullet(&self) -> bool {
        self.is_alien_bullet
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.sprite.contains(coordinate)
    }

    pub fn step(&mut self, delta_time: f32) -> Option<Coordinate> {
        let result = match self.is_alien_bullet {
            true => self.sprite.move_down(delta_time),
            false => self.sprite.move_up(delta_time),
        };
        return match result {
            Ok(state) => match state {
                State::Collided(coordinate) => {
                    //self.sprite.destroy();
                    Some(coordinate)
                }
                _ => None,
            },
            Err(error) => match error.kind() {
                ErrorKind::OutOfBounds => {
                    self.destroy();
                    None
                }
                _ => None,
            },
        };
    }

    pub fn is_destroyed(&self) -> bool {
        self.sprite.is_destroyed()
    }

    pub fn destroy(&mut self) {
        let _ = self.sprite.destroy();
    }
}

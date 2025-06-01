//! Obstacle sprites for the last stage of the end game.
//!
//! Obstacles in this game only spawn and drop downwards, trying to hit the player.
use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::entities::Bullet;
use crate::errors::{Error, ErrorKind};
use crate::utils;

use crate::OBSTACLE_SPEED;
use crate::OBSTACLE_WAIT_TIME;
use crate::SPEEDSTER_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Obstacle {
    sprite: Sprite,
    spawn_wait_time: f32,
    spawn_timer: Instant,
    wait_timer: Instant,
    wait_time: f32,
    destroy_on_contact: bool,
}

impl Obstacle {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        position: Vec<Coordinate>,
        velocity: f32,
    ) -> Result<Self, Error> {
        Ok(Self {
            spawn_wait_time: 0.0,
            spawn_timer: Instant::now(),
            wait_timer: Instant::now(),
            wait_time: OBSTACLE_WAIT_TIME,
            sprite: Sprite::new(engine, position, velocity, velocity)?,
            destroy_on_contact: false,
        })
    }

    pub fn set_spawn_wait_time(&mut self, s: f32) {
        self.spawn_wait_time = s;
    }

    /// Obstacles are not entities to be shot at and destroyed.
    /// Rather this function gives obstacles the capability to be
    /// destroyed when crashing or being shot at.
    pub fn to_destroy_on_contact(&mut self) {
        self.destroy_on_contact = true;
    }

    pub fn is_spawned(&self) -> bool {
        self.sprite.is_spawned()
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.spawn();
    }

    pub fn is_ready_to_spawn(&mut self) -> bool {
        // returns
        self.spawn_timer.elapsed().as_secs_f32() > self.spawn_wait_time
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        self.sprite.set_x_velocity(velocity);
        self.sprite.set_y_velocity(velocity);
    }

    pub fn set_wait_time(&mut self, duration: f32) {
        self.wait_time = duration;
    }

    pub fn is_wait_time_expired(&self) -> bool {
        if self.wait_timer.elapsed().as_secs_f32() >= self.wait_time {
            return true;
        }
        false
    }

    pub fn reset_wait_timer(&mut self) {
        self.wait_timer = Instant::now();
    }

    pub fn reset_spawn_timer(&mut self) {
        self.spawn_timer = Instant::now();
    }

    pub fn reset_timer(&mut self) {
        self.wait_timer = Instant::now();
        self.spawn_timer = Instant::now();
    }

    /// Because the obstacles drop downwards after a given time,
    /// let_drop() sets such timer to 0.0 indicating to drop immediately.
    pub fn let_drop(&mut self) {
        self.wait_time = 0.0;
    }

    /// The update function for obstacles
    pub fn step(&mut self, delta_time: f32) -> Option<Coordinate> {
        if self.is_destroyed() {
            return None;
        }
        if !self.is_wait_time_expired() {
            return None;
        }
        return match self.sprite.move_down(delta_time) {
            Ok(state) => match state {
                State::Collided(coordinate) => {
                    if self.destroy_on_contact {
                        self.destroy();
                    }
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

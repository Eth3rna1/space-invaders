//! Speedster implementation
use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::errors::{Error, ErrorKind};

use crate::SPEEDSTER_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Speedster {
    width: usize,
    is_dead: bool,
    sprite: Sprite,
    direction: Direction,
    is_initialized: bool,
}

impl Speedster {
    pub fn new(engine: Rc<RefCell<Engine>>, velocity: f32) -> Result<Self, Error> {
        let (width, height) = {
            let eng = engine.borrow();
            (eng.width, eng.width)
        };
        let direction = Direction::Left;
        let sprite = Sprite::new(
            engine,
            vec![(width - 1, 1), (width - 2, 1), (width - 3, 1)],
            velocity,
        )?;
        Ok(Self {
            width,
            sprite,
            direction,
            is_dead: false,
            is_initialized: false,
        })
    }

    pub fn x(&self) -> usize {
        self.sprite.far_left()
    }

    pub fn head(&self) -> Coordinate {
        (
            (self.sprite.far_right() - (self.sprite.far_right() - self.sprite.far_left()) / 2),
            self.sprite.far_bottom() + 1,
        )
    }

    pub fn is_dead(&self) -> bool {
        self.is_dead
    }

    pub fn is_spawned(&self) -> bool {
        self.sprite.is_spawned()
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn spawn_or_respawn(&mut self) -> Result<(), Error> {
        if self.sprite.is_spawned() {
            return Ok(());
        }
        return match self.sprite.spawn() {
            Ok(_) => {
                self.is_initialized = true;
                Ok(())
            }
            Err(err) => Err(err),
        };
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.sprite.contains(coordinate)
    }

    pub fn step(&mut self, delta_time: f32) -> Option<Coordinate> {
        if self.is_dead || !self.sprite.is_spawned() {
            return None;
        }
        // encapsulating the movement methods in a single function for more
        // consice code
        let mut movement_result: Box<dyn Fn(&mut Self) -> Result<State, Error>> =
            match self.direction {
                Direction::Left => Box::new(|s: &mut Self| s.sprite.move_left(delta_time)),
                Direction::Right => Box::new(|s: &mut Self| s.sprite.move_right(delta_time)),
            };
        // working with the single function
        return match movement_result(self) {
            Ok(state) => match state {
                // collided with a bullet
                State::Collided(coordinate) => {
                    self.is_dead = true;
                    Some(coordinate)
                }
                _ => None,
            },
            Err(error) => match error.kind() {
                ErrorKind::OutOfBounds => {
                    // inverting the direction
                    match self.direction {
                        Direction::Right => self.direction = Direction::Left,
                        Direction::Left => self.direction = Direction::Right,
                    }
                    self.destroy();
                    None
                }
                _ => None,
            },
        };
    }

    pub fn destroy(&mut self) {
        let _ = self.sprite.destroy();
    }
}

/*
    Aliens Sprite implementation
*/
use crate::engine::sprite::{Sprite, State};
use crate::engine::{Coordinate, Engine};
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct Aliens {
    sprites: Vec<Sprite>,
    direction: Direction,
    width: usize,
    length: usize,
}

impl Aliens {
    pub fn new(engine: Rc<RefCell<Engine>>, count: usize, velocity: usize) -> Result<Self, Error> {
        if count == 0 {
            return Err(Error::new(ErrorKind::Other, "Alien count cannot be 0"));
        }
        //let alien = Sprite::new(engine, position, velocity)?;
        let (width, length): (usize, usize) = {
            let eng = engine.borrow();
            (eng.width, eng.length)
        };
        Ok(Self {
            sprites: Vec::new(),
            direction: Direction::Right,
            width,
            length,
        })
    }

    pub fn spawn(&mut self) {
        for alien in self.sprites.iter_mut() {
            alien.spawn();
        }
    }

    pub fn step(&mut self) -> Result<State, Error> {
        match self.direction {
            Direction::Right => {
                if self.sprites[0].bounding_box.far_right == self.width {
                    self.direction = Direction::Left;
                    return Err(Error::new(
                        ErrorKind::OutOfBounds,
                        "Hit the far right boundry",
                    ));
                }
                for sprite in self.sprites.iter_mut() {
                    let _ = sprite.move_right(); // collision logic has not been implemented yet
                }
            }
            Direction::Left => {
                if self.sprites[self.sprites.len() - 1].bounding_box.far_left == 0 {
                    self.direction = Direction::Right;
                    return Err(Error::new(
                        ErrorKind::OutOfBounds,
                        "Hit the far left boundry",
                    ));
                }
                for sprite in self.sprites.iter_mut() {
                    let _ = sprite.move_left(); // collision logic has not been implemented yet
                }
            }
        }
        Ok(State::Moved)
    }
}

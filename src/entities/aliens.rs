/*
    Aliens Sprite implementation
*/
use crate::engine::sprite::{Sprite, State};
use crate::engine::{Coordinate, Engine};
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;
use std::slice::{Iter, IterMut};

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
    pub fn new(engine: Rc<RefCell<Engine>>, count: usize, velocity: f64) -> Result<Self, Error> {
        if count == 0 {
            return Err(Error::new(ErrorKind::Other, "Alien count cannot be 0"));
        }
        let (width, length): (usize, usize) = {
            let eng = engine.borrow();
            (eng.width, eng.height)
        };
        let mut sprites: Vec<Sprite> = Vec::new();
        let delta = width / count;
        let mut c = 0;
        for row in [4, 7, 10] {
            for col in 0..count {
                if c + delta >= width {
                    break;
                }
                let position = vec![
                    //(c, row),
                    (c + 1, row),
                    //(c + 2, row),
                    (c, row - 1),
                    //(c + 1, row - 1),
                    (c + 2, row - 1),
                ];
                sprites.push(Sprite::new(engine.clone(), position, velocity)?);
                c += delta;
            }
            c = 0
        }
        Ok(Self {
            sprites,
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

    pub fn iter(&self) -> Iter<'_, Sprite> {
        self.sprites.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Sprite> {
        self.sprites.iter_mut()
    }

    pub fn step(&mut self) -> Result<State, Error> {
        match self.direction {
            Direction::Right => {
                if self.sprites[self.sprites.len() - 1].bounding_box.far_right == self.width - 1 {
                    self.direction = Direction::Left;
                    return Err(Error::new(
                        ErrorKind::OutOfBounds,
                        "Hit the far right boundry",
                    ));
                }
                for sprite in self.sprites.iter_mut() {
                    let result = sprite.move_right();
                    if let Ok(State::Collided(_)) = result {
                        sprite.destroy(); // automatically dies
                    }
                }
            }
            Direction::Left => {
                if self.sprites[0].bounding_box.far_left == 0 {
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

/*
    Shooter Sprite implementation
*/
use crate::engine::sprite::{Sprite, State};
use crate::engine::{Coordinate, Engine};
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Shooter {
    sprite: Sprite,
}

impl Shooter {
    pub fn new(engine: Rc<RefCell<Engine>>, velocity: usize) -> Result<Self, Error> {
        let position: Vec<Coordinate> = {
            let eng = engine.borrow();
            vec![
                (eng.width / 2, eng.length - (eng.length / 7)),
                (eng.width / 2, eng.length - (eng.length / 7) - 1),
                (eng.width / 2 + 1, eng.length - (eng.length / 7)),
                (eng.width / 2 - 1, eng.length - (eng.length / 7)),
            ]
        };
        Ok(Self {
            sprite: Sprite::new(engine, position, velocity)?,
        })
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.spawn();
    }

    pub fn step(&mut self, key: &str) -> Result<State, Error> {
        match key {
            "right" => self.sprite.move_right(),
            "left" => self.sprite.move_left(),
            " " => Err(Error::new(
                ErrorKind::Other,
                "Have not implemented bullet logic yet!",
            )),
            _ => Err(Error::new(ErrorKind::Other, "Unknown key was pressed")),
        }
    }
}

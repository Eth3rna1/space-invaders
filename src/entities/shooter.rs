/*
    Shooter Sprite implementation
*/
use crate::engine::sprite::{Sprite, State};
use crate::engine::{Coordinate, Engine};
use crate::entities::Bullet;
use crate::errors::{Error, ErrorKind};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Shooter {
    sprite: Sprite,
    //has_been_spawned: bool
}

impl Shooter {
    pub fn new(engine: Rc<RefCell<Engine>>, velocity: usize) -> Result<Self, Error> {
        let position: Vec<Coordinate> = {
            let eng = engine.borrow();
            vec![
                (eng.width / 2 + 2, eng.length - (eng.length / 7)),
                (eng.width / 2 - 2, eng.length - (eng.length / 7)),
                (eng.width / 2, eng.length - (eng.length / 7)),
                (eng.width / 2, eng.length - (eng.length / 7) - 1),
                (eng.width / 2 + 1, eng.length - (eng.length / 7)),
                (eng.width / 2 - 1, eng.length - (eng.length / 7)),
            ]
        };
        Ok(Self {
            sprite: Sprite::new(engine, position, velocity)?,
            //has_been_spawned: false
        })
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.spawn();
        //self.has_been_spawned = true;
    }

    pub fn step(&mut self, key: &str) -> Result<State, Error> {
        //if !self.has_been_spawned {
        //    return Err(Error::new(ErrorKind::Other, "Shooter has not been spawned"))
        //}
        match key {
            "right" => self.sprite.move_right(),
            "left" => self.sprite.move_left(),
            // the following line is needed so the player can actually shoot
            " " => Ok(State::Null),
            _ => Err(Error::new(ErrorKind::Other, "Unknown key was pressed")),
        }
    }

    pub fn head(&self) -> Coordinate {
        let far_left = self.sprite.bounding_box.far_left;
        let far_right = self.sprite.bounding_box.far_right;
        (
            far_left + ((far_right - far_left) / 2),
            self.sprite.bounding_box.far_top - 2,
        )
    }
}

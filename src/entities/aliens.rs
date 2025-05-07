use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::errors::{Error, ErrorKind};
use crate::utils;

use crate::ALIEN_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;
//use std::slice::IterMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Left,
}

/// returns a reference to the farthest left most alien
pub fn farthest_left_alien<'a>(aliens: &'a [Alien]) -> &'a Alien {
    let mut index = 0;
    let mut lowest = aliens[0].far_left();
    for i in 0..aliens.len() {
        if aliens[i].far_left() < lowest {
            index = i;
            lowest = aliens[i].far_left();
        }
    }
    &aliens[index]
}

#[derive(Debug, Clone)]
pub struct Alien {
    sprite: Sprite,
    velocity: f32,
    width: usize,
}

impl Alien {
    pub fn new(
        engine: Rc<RefCell<Engine>>,
        position: Vec<Coordinate>,
        velocity: f32,
    ) -> Result<Self, Error> {
        let width = { engine.borrow().width };
        Ok(Self {
            width,
            velocity,
            sprite: Sprite::new(engine, position, velocity, velocity)?,
        })
    }

    pub fn x(&self) -> usize {
        self.sprite.far_left()
    }

    pub fn far_right(&self) -> usize {
        self.sprite.far_right()
    }

    pub fn far_left(&self) -> usize {
        self.sprite.far_left()
    }

    pub fn far_bottom(&self) -> usize {
        self.sprite.far_bottom()
    }

    pub fn spawn(&mut self) -> Result<(), Error> {
        let _ = self.sprite.spawn()?;
        Ok(())
    }

    //pub fn offset_x(&mut self, offset: f32) {
    //    self.sprite.offset_exact_x(offset);
    //}

    pub fn move_y(&mut self, step: i32) -> Result<State, Error> {
        self.sprite.move_relative_y(step)
    }

    pub fn step(&mut self, step: i32) -> Option<Coordinate> {
        if self.sprite.is_destroyed() {
            return None;
        }
        //self.sprite.offset_exact_x(step as f32);
        return match self.sprite.move_relative_x(step) {
            Ok(state) => {
                if let State::Collided(coordinate) = state {
                    Some(coordinate)
                } else {
                    None
                }
            }
            Err(_) => None,
        };
    }

    pub fn is_destroyed(&mut self) -> bool {
        self.sprite.is_destroyed()
    }

    pub fn head(&self) -> Coordinate {
        (
            (self.sprite.far_right() - (self.sprite.far_right() - self.sprite.far_left()) / 2),
            self.sprite.far_bottom() + 1,
        )
    }

    pub fn destroy(&mut self) {
        let _ = self.sprite.destroy();
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.sprite.contains(coordinate)
    }
}

pub fn find_alien_and_destroy(aliens: &mut Vec<Alien>, coordinate: Coordinate) -> bool {
    for i in 0..aliens.len() {
        if aliens[i].contains(coordinate) {
            aliens[i].destroy();
            let _ = aliens.remove(i);
            return true;
        }
    }
    false
}

pub fn spawn_aliens(
    engine: Rc<RefCell<Engine>>,
    count: usize,
    velocity: f32,
) -> Result<Vec<Alien>, Error> {
    let eng = engine.borrow();
    let mut collector: Vec<Alien> = Vec::new();
    let width = 4; // sprite width
    let alien_width = 3 * width; // total width of each alien (3 sprites per alien)
    let space_between = (eng.width - alien_width * count) / (count + 1); // Calculate space between aliens

    // Loop to generate alien rows
    for row in [4, 8, 12] {
        let mut pointer = 0;
        while pointer + alien_width <= eng.width {
            // Ensure we stay within bounds
            let position = vec![
                (pointer, row),
                (pointer + 1, row),
                (pointer + 2, row),
                (pointer, row + 1),
                //(pointer + 1, row + 1),
                (pointer + 2, row + 1),
            ];
            collector.push(Alien::new(engine.clone(), position, velocity)?);
            pointer += alien_width + space_between; // Update pointer to next position
                                                    //pointer += space_between;
        }
    }
    Ok(collector)
}

/// returns a reference to the farthest right most alien
pub fn farthest_right_alien<'a>(aliens: &'a [Alien]) -> &'a Alien {
    let mut index = 0;
    let mut alpha = aliens[0].far_right();
    for i in 0..aliens.len() {
        if aliens[i].far_right() > alpha {
            index = i;
            alpha = aliens[i].far_right();
        }
    }
    &aliens[index]
}

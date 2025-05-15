//! Zig Zag event for stage 3
use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::entities::Bullet;
use crate::entities::Obstacle;
use crate::entities::Speedster;
use crate::errors::{Error, ErrorKind};
use crate::utils;

use crate::OBSTACLE_SPEED;
use crate::OBSTACLE_WAIT_TIME;
use crate::SPEEDSTER_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

// perhapss this
#[derive(Debug, Clone)]
enum XDirection {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct ZigZagEvent {
    speedster: Rc<RefCell<Sprite>>,
    is_finished: bool,
    direction: XDirection,
    //wall_hit_counter: usize,
    //wall_hits_before_moving: usize,
}

impl ZigZagEvent {
    pub fn new(speedster: Rc<RefCell<Sprite>>, width: usize) -> Self {
        let mut sprite = speedster.borrow_mut();
        //sprite.destroy();
        //{
        //    let mut coordinates = sprite.coordinates_mut();
        //    coordinates[0] = (width - 1, 1);
        //    coordinates[1] = (width - 2, 1);
        //    coordinates[2] = (width - 3, 1);
        //    sprite.recalc_bounding_box();
        //}
        //let _ = sprite.spawn();
        sprite.set_x_velocity(100.0);
        sprite.set_y_velocity(100.0);
        sprite.recalc_bounding_box();
        drop(sprite);
        Self {
            //width,
            speedster,
            direction: XDirection::Left,
            is_finished: false,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    fn move_down(&mut self, delta_time: f32) -> Option<Coordinate> {
        let mut speedster = self.speedster.borrow_mut();
        match speedster.move_down(delta_time) {
            Ok(state) => match state {
                State::Collided(coordinate) => {
                    self.is_finished = true;
                    return Some(coordinate)
                },
                _ => (),
            },
            Err(error) => match error.kind() {
                ErrorKind::OutOfBounds => {
                    self.is_finished = true;
                    let _ = speedster.destroy();
                }
                _ => (),
            },
        };
        // let result = match self.direction {
        //     XDirection::Left => speedster.move_relative_x()
        //     XDirection::Right => {}
        // }
        None
    }

    fn position(&self) -> usize {
        let sprite = self.speedster.borrow();
        sprite.far_right() - (sprite.far_right() - sprite.far_left()) / 2
    }

    fn has_obstruction(&self) -> bool {
        let x = self.position();
        let speedster = self.speedster.borrow();
        let engine_ptr = speedster.engine();
        let engine = engine_ptr.borrow();
        for y in speedster.far_bottom() + 1..engine.height {
            if engine.is_on(&(x, y)) {
                return true;
            }
        }
        false
    }

    pub fn step(&mut self, delta_time: f32) -> Option<Coordinate> {
        let position = self.position();
        if position % 33 == 0 && position != 0 {
            let result = self.move_down(delta_time);
            if result.is_some() {
                return result;
            }
        }
        let mut speedster = self.speedster.borrow_mut();
        let result: Result<State, Error> = match self.direction {
            XDirection::Left => speedster.move_left(delta_time),
            XDirection::Right => speedster.move_right(delta_time),
        };
        return match result {
            Ok(state) => match state {
                State::Collided(coordinate) => {
                    self.is_finished = true;
                    Some(coordinate)
                },
                _ => None,
            },
            Err(error) => match error.kind() {
                ErrorKind::OutOfBounds => {
                    match self.direction {
                        XDirection::Left => self.direction = XDirection::Right,
                        XDirection::Right => self.direction = XDirection::Left,
                    }
                    None
                }
                _ => None,
            },
        };
    }

    pub fn destroy(&mut self) {
        let mut speedster = self.speedster.borrow_mut();
        speedster.destroy();
    }
}

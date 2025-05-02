use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::entities::Bullet;
use crate::entities::Obstacle;
use crate::errors::{Error, ErrorKind};
use crate::utils;

use crate::OBSTACLE_SPEED;
use crate::OBSTACLE_WAIT_TIME;
use crate::SPEEDSTER_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct SnowEvent {
    engine: Rc<RefCell<Engine>>,
    width: usize,
    delay_timer: Instant,
    obstacles: Vec<Obstacle>,
    spawn_time_delay: f32,
    total_rows: usize,
    rows_spawned: usize,
    col_d: usize,
    is_finished: bool,
}

impl SnowEvent {
    pub fn new(engine: Rc<RefCell<Engine>>) -> Self {
        let width = { engine.borrow().width };
        Self {
            engine,
            width,
            rows_spawned: 0,
            //total_rows: 10,
            total_rows: 5,
            delay_timer: Instant::now(),
            spawn_time_delay: 1.7,
            obstacles: Vec::new(),
            col_d: 9,
            is_finished: false,
        }
    }

    pub fn spawn_row(&mut self) {
        for i in 0..self.width - 1 {
            if i == 0 {
                continue;
            }
            if i % self.col_d != 0 {
                continue;
            }
            if let Ok(mut obstacle) = Obstacle::new(self.engine.clone(), vec![(i, 4)], 3.0) {
                obstacle.spawn();
                obstacle.to_destroy_on_contact();
                obstacle.set_wait_time(0.0); // no need to make it wait
                self.obstacles.push(obstacle);
            }
        }
        self.rows_spawned += 1;
        match self.col_d {
            9 => self.col_d = 7,
            7 => self.col_d = 9,
            _ => panic!("CRITICAL ERROR HAS OCCURED IN THE SNOW EVENT"),
        }
        self.delay_timer = Instant::now();
    }

    pub fn is_ready_to_spawn_row(&self) -> bool {
        if self.rows_spawned == self.total_rows {
            return false;
        }
        if self.delay_timer.elapsed().as_secs_f32() < self.spawn_time_delay {
            return false;
        }
        true
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn deallocate_destroyed_obstacles(&mut self) {
        for i in (0..self.obstacles.len()).rev() {
            if self.obstacles[i].is_destroyed() {
                let _ = self.obstacles.remove(i);
            }
        }
    }

    pub fn move_obstacles(&mut self, delta_time: f32) -> Option<Coordinate> {
        for flake in self.obstacles.iter_mut() {
            let result = flake.step(delta_time);
            if result.is_some() {
                flake.destroy();
                return result;
            }
        }
        None
    }

    pub fn step(&mut self, delta_time: f32) -> Option<Coordinate> {
        if self.rows_spawned == self.total_rows && self.obstacles.is_empty() {
            self.is_finished = true;
        }
        if self.is_finished {
            return None;
        }
        if self.is_ready_to_spawn_row() {
            self.spawn_row();
        }
        self.deallocate_destroyed_obstacles();
        self.move_obstacles(delta_time)
    }
}

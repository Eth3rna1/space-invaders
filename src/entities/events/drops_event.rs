//! Drops events
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

#[derive(Debug, Clone)]
pub struct DropsEvent {
    obstacles: Vec<Obstacle>,
    is_finished: bool,
    is_initialized: bool,
}

impl DropsEvent {
    pub fn new(engine: Rc<RefCell<Engine>>) -> Self {
        let (width, height) = {
            let eng = engine.borrow();
            (eng.width, eng.height)
        };
        let mut obstacles: Vec<Obstacle> = Vec::new();
        for i in 0..width {
            if i % 3 != 0 {
                continue;
            }
            if i + 2 >= width {
                continue;
            }
            if i <= width / 2 {
                // first half
                if let Ok(mut obstacle) = Obstacle::new(
                    engine.clone(),
                    vec![(i, height / 2), (i + 1, height / 2), (i + 2, height / 2)],
                    OBSTACLE_SPEED,
                ) {
                    obstacle.set_wait_time(2.0 + i as f32 / 10.0);
                    //obstacle.set_spawn_wait_time(2.0 + i as f32 / 7.0);
                    obstacles.push(obstacle);
                }
            } else {
                if let Ok(mut obstacle) = Obstacle::new(
                    engine.clone(),
                    vec![(i, height / 2), (i + 1, height / 2), (i + 2, height / 2)],
                    OBSTACLE_SPEED,
                ) {
                    obstacle.set_wait_time((width as f32 - i as f32) / 10.0 + 2.0);
                    //obstacle.set_spawn_wait_time(2.0 + i as f32 / 7.0);
                    obstacles.push(obstacle);
                }
            }
        }
        Self {
            obstacles,
            is_finished: false,
            is_initialized: false,
        }
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
        for o in self.obstacles.iter_mut() {
            let result = o.step(delta_time);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    pub fn reset_wait_timers(&mut self) {
        for o in self.obstacles.iter_mut() {
            o.reset_wait_timer();
        }
    }

    pub fn step(&mut self, delta_time: f32) -> Option<Coordinate> {
        if self.obstacles.is_empty() {
            self.is_finished = true;
            return None;
        }
        if !self.is_initialized {
            for o in self.obstacles.iter_mut() {
                //o.reset_wait_timer();
                if o.is_ready_to_spawn() {
                    o.spawn();
                }
            }
            self.reset_wait_timers();
            if self.obstacles.iter().all(|o| o.is_spawned()) {
                self.is_initialized = true;
            }
            return None;
        }
        self.deallocate_destroyed_obstacles();
        self.move_obstacles(delta_time)
    }
}

//! Speedster implementation
use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::entities::events::{DropsEvent, SnowEvent, ZigZagEvent};
use crate::entities::Bullet;
use crate::entities::Obstacle;
use crate::errors::{Error, ErrorKind};
use crate::utils;
use crate::utils::rand_num;

use crate::BULLET_STEP_PER_DELTA;
use crate::OBSTACLE_SPEED;
use crate::OBSTACLE_WAIT_TIME;
use crate::SPEEDSTER_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
enum XDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum YDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EndGameState {
    Stage1,
    Stage2,
    Stage3,
    Over,
}

#[derive(Debug, Clone)]
pub struct Speedster {
    width: usize,
    height: usize,
    sprite: Rc<RefCell<Sprite>>,
    state: EndGameState,
    snow_event: SnowEvent,
    xdirection: XDirection,
    ydirection: YDirection,
    drops_event: DropsEvent,
    obstacles: Vec<Obstacle>,
    stage_3_initialized: bool,
    zigzag_event: ZigZagEvent,
    engine: Rc<RefCell<Engine>>,
}

impl Speedster {
    pub fn new(engine: Rc<RefCell<Engine>>, velocity: f32) -> Result<Self, Error> {
        let (width, height) = {
            let eng = engine.borrow();
            (eng.width, eng.height - eng.height / 3)
        };
        let mut sprite = Rc::new(RefCell::new(Sprite::new(
            engine.clone(),
            vec![(width - 1, 1), (width - 2, 1), (width - 3, 1)],
            velocity,
        )?));
        Ok(Self {
            snow_event: SnowEvent::new(engine.clone()),
            zigzag_event: ZigZagEvent::new(sprite.clone(), width),
            drops_event: DropsEvent::new(engine.clone()),
            width,
            height,
            sprite,
            engine,
            obstacles: Vec::new(),
            state: EndGameState::Stage1,
            //state: EndGameState::Stage3,
            xdirection: XDirection::Left,
            ydirection: YDirection::Down,
            stage_3_initialized: false,
        })
    }

    pub fn set_velocity(&mut self, v: f32) {
        self.sprite.borrow_mut().set_velocity(v);
    }

    pub fn x(&self) -> usize {
        self.sprite.borrow().far_left()
    }

    pub fn head(&self) -> Coordinate {
        let sprite = self.sprite.borrow();
        (
            (sprite.far_right() - (sprite.far_right() - sprite.far_left()) / 2),
            sprite.far_bottom() + 1,
        )
    }

    pub fn is_spawned(&self) -> bool {
        self.sprite.borrow().is_spawned()
    }

    pub fn next_stage(&mut self) {
        match self.state {
            EndGameState::Stage1 => self.state = EndGameState::Stage2,
            EndGameState::Stage2 => self.state = EndGameState::Stage3,
            EndGameState::Stage3 => self.state = EndGameState::Over,
            EndGameState::Over => return,
        }
    }

    pub fn spawn(&mut self) {
        let _ = self.sprite.borrow_mut().spawn();
    }

    pub fn contains(&self, coordinate: Coordinate) -> bool {
        self.sprite.borrow().contains(coordinate)
    }

    pub fn reset_position(&mut self) {
        self.destroy();
        self.xdirection = XDirection::Left;
        self.ydirection = YDirection::Down;
        let mut sprite = self.sprite.borrow_mut();
        let mut coordinates = sprite.coordinates_mut();
        coordinates[0] = (self.width - 1, 1);
        coordinates[1] = (self.width - 2, 1);
        coordinates[2] = (self.width - 3, 1);
        sprite.recalc_bounding_box();
    }

    pub fn stage_1(&mut self, delta_time: f32) -> Option<Coordinate> {
        let mut sprite = self.sprite.borrow_mut();
        // encapsulating the movement methods in a single function for more
        // consice code
        let mut movement_result: Result<State, Error> = match self.xdirection {
            XDirection::Left => sprite.move_left(delta_time),
            XDirection::Right => sprite.move_right(delta_time),
        };
        // working with the single function
        return match movement_result {
            Ok(state) => match state {
                // collided with a bullet
                State::Collided(coordinate) => Some(coordinate),
                _ => None,
            },
            Err(error) => match error.kind() {
                ErrorKind::OutOfBounds => {
                    // inverting the direction
                    match self.xdirection {
                        XDirection::Right => self.xdirection = XDirection::Left,
                        XDirection::Left => self.xdirection = XDirection::Right,
                    }
                    None
                }
                _ => None,
            },
        };
    }

    pub fn stage_2(&mut self, delta_time: f32) -> Option<Coordinate> {
        let mut sprite = self.sprite.borrow_mut();
        let result: Result<State, Error> = match self.xdirection {
            XDirection::Left => {
                let fl = sprite.far_left();
                if fl % 25 == 0 && fl != 0 && fl != self.width - 1 {
                    match self.ydirection {
                        YDirection::Up => {
                            if sprite.far_top() <= 1 {
                                self.ydirection = YDirection::Down;
                                // continuing in the x axis
                                sprite.move_left(delta_time)
                            } else {
                                sprite.move_up(delta_time)
                            }
                        }
                        YDirection::Down => {
                            if sprite.far_bottom() >= self.height {
                                self.ydirection = YDirection::Up;
                                // continuing in the x axis
                                sprite.move_left(delta_time)
                            } else {
                                sprite.move_down(delta_time)
                            }
                        }
                    }
                } else {
                    sprite.move_left(delta_time)
                }
            }
            XDirection::Right => {
                let fr = sprite.far_right();
                if fr % 25 == 0 && fr != 0 && fr != self.width - 1 {
                    match self.ydirection {
                        YDirection::Up => {
                            if sprite.far_top() <= 1 {
                                self.ydirection = YDirection::Down;
                                // continuing in the x position
                                sprite.move_right(delta_time)
                            } else {
                                sprite.move_up(delta_time)
                            }
                        }
                        YDirection::Down => {
                            if sprite.far_bottom() >= self.height {
                                self.ydirection = YDirection::Up;
                                // continuing in the x position
                                sprite.move_right(delta_time)
                            } else {
                                sprite.move_down(delta_time)
                            }
                        }
                    }
                } else {
                    sprite.move_right(delta_time)
                }
            }
        };
        return match result {
            Ok(state) => match state {
                State::Collided(coordinate) => Some(coordinate),
                _ => None,
            },
            Err(error) => match error.kind() {
                ErrorKind::OutOfBounds => {
                    match self.xdirection {
                        XDirection::Left => self.xdirection = XDirection::Right,
                        XDirection::Right => self.xdirection = XDirection::Left,
                    }
                    None
                }
                _ => None,
            },
        };
    }

    pub fn s3_place_speedster_in_center_x(&mut self) {
        self.destroy();
        let mut sprite = self.sprite.borrow_mut();
        let mut coordinates = sprite.coordinates_mut();
        coordinates[0].0 = (self.width / 2) - 1;
        coordinates[1].0 = (self.width / 2) - 0;
        coordinates[2].0 = (self.width / 2) + 1;
        sprite.recalc_bounding_box();
    }

    //fn deallocate_destroyed_obstacles(&mut self) {
    //    for i in (0..self.obstacles.len()).rev() {
    //        if self.obstacles[i].is_destroyed() {
    //            let _ = self.obstacles.remove(i);
    //        }
    //    }
    //}

    fn place_guard_on_speedster(&mut self) -> Result<Obstacle, Error> {
        let mut guard_position: Vec<Coordinate> = Vec::new();
        // procedurally constructing the guard
        let sprite = self.sprite.borrow();
        let fl = sprite.far_left();
        let fr = sprite.far_right();
        if fl as i32 - 4 >= 0 {
            guard_position.extend([(fl - 4, 0), (fl - 4, 1), (fl - 4, 2), (fl - 4, 3)]);
        }
        for i in fl - 4..=fr + 4 {
            guard_position.push((i, 3));
        }
        if fr + 2 <= self.width - 1 {
            guard_position.extend([(fr + 4, 0), (fr + 4, 1), (fr + 4, 2), (fr + 4, 3)]);
        }
        let mut obstacle = Obstacle::new(self.engine.clone(), guard_position, OBSTACLE_SPEED)?;
        obstacle.set_wait_time(1000.0); // making the obstacle last
        obstacle.spawn();
        Ok(obstacle)
    }

    fn run_obstacles(&mut self, delta_time: f32) -> Option<Coordinate> {
        for o in self.obstacles.iter_mut() {
            let result = o.step(delta_time);
            if result.is_some() {
                return result;
            }
        }
        None
    }

    fn has_obstruction(&self, bullets: &[Bullet]) -> bool {
        let sprite = self.sprite.borrow();
        let x = sprite.far_right() - (sprite.far_right() - sprite.far_left()) / 2;
        let speedster = self.sprite.borrow();
        let engine_ptr = speedster.engine();
        let engine = engine_ptr.borrow();
        for y in speedster.far_bottom() + 1..engine.height {
            if engine.is_on(&(x, y)) && bullets.iter().all(|b| !b.contains((x, y))) {
                return true;
            }
        }
        false
    }

    fn eval_with_bullets(&self, bullets: &[Bullet], coordinate: &Coordinate) -> Option<Coordinate> {
        for b in bullets.iter() {
            if b.is_alien_bullet() && b.contains(*coordinate) {
                return None;
            }
        }
        Some(*coordinate)
    }

    pub fn stage_3(&mut self, delta_time: f32, bullets: &mut Vec<Bullet>) -> Option<Coordinate> {
        //let mut guard: Obstacle;
        if !self.stage_3_initialized {
            self.s3_place_speedster_in_center_x();
            if let Ok(guard) = self.place_guard_on_speedster() {
                self.obstacles.push(guard);
            } else {
                return None;
            }
            self.spawn();
            self.stage_3_initialized = true;
        }
        if !self.snow_event.is_finished() {
            if let Some(coordinate) = self.snow_event.step(delta_time) {
                //return Some(coordinate);
                return self.eval_with_bullets(&bullets, &coordinate);
            }
        } else if self.snow_event.is_finished() && !self.obstacles[0].is_destroyed() {
            self.obstacles[0].let_drop(); // just a trigger function that sets the wait time to 0.0
        } else if !self.drops_event.is_finished() {
            if let Some(coordinate) = self.drops_event.step(delta_time) {
                //return Some(coordinate);
                return self.eval_with_bullets(&bullets, &coordinate);
            }
        } else if !self.zigzag_event.is_finished() {
            if let Some(coordinate) = self.zigzag_event.step(delta_time) {
                //return Some(coordinate);
                return self.eval_with_bullets(&bullets, &coordinate);
            }
            if self.has_obstruction(&bullets) {
                if let Ok(b) = Bullet::new(
                    self.sprite.borrow().engine(),
                    self.head(),
                    BULLET_STEP_PER_DELTA,
                ) {
                    let mut bullet = b.to_alien_bullet();
                    bullet.spawn();
                    bullets.push(bullet);
                }
            }
        }
        self.run_obstacles(delta_time)
    }

    pub fn step(&mut self, delta_time: f32, bullets: &mut Vec<Bullet>) -> Option<Coordinate> {
        let result: Option<Coordinate> = match self.state {
            EndGameState::Stage1 => self.stage_1(delta_time),
            EndGameState::Stage2 => self.stage_2(delta_time),
            EndGameState::Stage3 => self.stage_3(delta_time, bullets),
            EndGameState::Over => None,
        };
        if self.has_obstruction(&bullets) && self.stage_3_phase() != 3 {
            if let Ok(b) = Bullet::new(
                self.sprite.borrow().engine(),
                self.head(),
                BULLET_STEP_PER_DELTA,
            ) {
                let mut bullet = b.to_alien_bullet();
                bullet.spawn();
                bullets.push(bullet);
            }
        }
        if let Some(coordinate) = result {
            for b in bullets.iter_mut() {
                if b.is_alien_bullet() {
                    continue;
                }
                if b.contains(coordinate) {
                    b.destroy();
                    return result;
                }
            }
        }
        result
    }

    pub fn was_hit(&mut self) {
        self.destroy();
        self.reset_position();
        self.next_stage();
    }

    pub fn is_dead(&self) -> bool {
        self.state == EndGameState::Over
    }

    pub fn destroy(&mut self) {
        let mut sprite = self.sprite.borrow_mut();
        let _ = sprite.destroy();
    }

    pub fn stages(&self) -> u8 {
        3
    }

    pub fn stage_3_phase(&self) -> u8 {
        if self.state != EndGameState::Stage3 {
            return 0;
        }
        if self.zigzag_event.is_finished() {
            return 3;
        } else if self.drops_event.is_finished() {
            return 3;
        } else if self.snow_event.is_finished() {
            return 2;
        }
        1
    }

    pub fn stages_completed(&self) -> u8 {
        return match self.state {
            EndGameState::Stage1 => 0,
            EndGameState::Stage2 => 1,
            EndGameState::Stage3 => 2,
            EndGameState::Over => 3,
        };
    }
}

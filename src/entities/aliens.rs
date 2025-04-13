use crate::engine::sprite::Sprite;
use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::errors::{Error, ErrorKind};

use crate::ALIEN_STEP_PER_DELTA;

use std::cell::RefCell;
use std::rc::Rc;
//use std::slice::IterMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct Aliens {
    aliens: Vec<Sprite>,
    direction: Direction,
    velocity: f32,
    width: usize,
}

impl Aliens {
    pub fn new(engine: Rc<RefCell<Engine>>, count: usize, velocity: f32) -> Result<Self, Error> {
        let mut aliens: Vec<Sprite> = {
            let eng = engine.borrow();
            let mut collector: Vec<Sprite> = Vec::new();
            let width = 4; // sprite width
            let alien_width = 3 * width; // total width of each alien (3 sprites per alien)
            let space_between = (eng.width - alien_width * count) / (count + 1); // Calculate space between aliens

            // Loop to generate alien rows
            for row in [4, 8, 12] {
                let mut pointer = space_between; // Start the pointer at space_between
                while pointer + alien_width <= eng.width {
                    // Ensure we stay within bounds
                    let position = vec![
                        (pointer, row),
                        (pointer + 1, row),
                        (pointer + 2, row),
                        (pointer, row + 1),
                        (pointer + 1, row + 1),
                        (pointer + 2, row + 1),
                    ];
                    collector.push(Sprite::new(engine.clone(), position, velocity)?);
                    pointer += alien_width + space_between; // Update pointer to next position
                                                            //pointer += space_between;
                }
            }
            collector
        };
        Ok(Self {
            aliens,
            velocity,
            width: { engine.borrow().width },
            direction: Direction::Right,
        })
    }

    pub fn spawn(&mut self) {
        for alien in self.aliens.iter_mut() {
            let _ = alien.spawn();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.aliens.is_empty() || self.aliens.iter().all(|a| a.is_destroyed())
    }

    fn farthest_right(&self) -> usize {
        //self.aliens[self.aliens.len() - 1].far_right()
        let mut f = self.aliens[0].far_right();
        for alien in self.aliens.iter() {
            let _fr = alien.far_right();
            if _fr > f {
                f = _fr;
            }
        }
        f
    }

    fn farthest_left(&self) -> usize {
        //self.aliens[0].far_left()
        let mut f = self.aliens[self.aliens.len() - 1].far_left();
        for alien in self.aliens.iter() {
            let _fr = alien.far_left();
            if _fr < f {
                f = _fr;
            }
        }
        f
    }

    pub fn find_and_destroy(&mut self, coordinate: Coordinate) -> bool {
        for i in 0..self.aliens.len() {
            if self.aliens[i].contains(coordinate) {
                let _ = self.aliens[i].destroy();
                self.aliens.remove(i);
                return true;
            }
        }
        false
    }

    pub fn step(&mut self, delta_time: f32) -> Option<Coordinate> {
        if self.aliens.is_empty() {
            return None;
        }
        // Giving a custom step to all the alien sprites
        // because there's an issue that aliens in the extreme
        // boundries modify their step, leading to inconsistent steps
        // among all aliens. When an alien in an extreme side
        // goes out of bounds, I want the custom step to apply to all
        // the aliens, not just the ones in the left and right boundry
        //
        // I want to avoid using the move_left() and move_right() methods
        // because they encapsulate logic that I don't have access to. In this case,
        // I created a move_relative_x() method where an arbitrary step must be
        // given. The downside is that I'll have to manually take care of the other internal
        // variables like updating the exact_x variable, and the logic that happens within.
        // A small price for in return, more control over the sprites.
        let offset: f32 = delta_time * self.velocity;
        let step: i32 = {
            let x = self.aliens[0].exact_x();
            // obtaining the whole number difference
            let abs_step = (x + offset) as usize - x as usize;
            match self.direction {
                Direction::Left => {
                    if self.farthest_left() as i32 - (abs_step as i32) < 0 {
                        // modifying the step because otherwise, it will to
                        // an out of bounds error
                        self.farthest_right() as i32
                    } else {
                        // turning the step into a negative number
                        // to signify moving left
                        0 - abs_step as i32
                    }
                }
                Direction::Right => {
                    if self.farthest_right() as i32 + (abs_step as i32) > (self.width - 1) as i32 {
                        // modifying the step because otherwise, the sprite
                        // will step out of bounds
                        (self.width - self.farthest_left() - 1) as i32
                    } else {
                        abs_step as i32
                    }
                }
            }
        };
        // making the offset to negative for left movement
        let neg_offset = 0.0 - offset;
        let mut coordinate_if_collided: Option<Coordinate> = None;
        let mut aliens_index_to_destroy: Vec<usize> = Vec::new();
        match self.direction {
            Direction::Left => {
                if self.farthest_left() == 0 {
                    self.direction = Direction::Right;
                    return None;
                }
                for i in 0..self.aliens.len() {
                    let mut alien = &mut self.aliens[i];
                    alien.offset_exact_x(neg_offset);
                    match alien.move_relative_x(step) {
                        Ok(state) => match state {
                            State::Collided(coordinate) => {
                                coordinate_if_collided = Some(coordinate);
                                aliens_index_to_destroy.push(i);
                            }
                            _ => (),
                        },
                        Err(_) => (),
                    }
                }
            }
            Direction::Right => {
                if self.farthest_right() == self.width - 1 {
                    self.direction = Direction::Left;
                    return None;
                }
                for i in 0..self.aliens.len() {
                    let mut alien = &mut self.aliens[i];
                    alien.offset_exact_x(offset);
                    match alien.move_relative_x(step) {
                        Ok(state) => match state {
                            State::Collided(coordinate) => {
                                coordinate_if_collided = Some(coordinate);
                                aliens_index_to_destroy.push(i);
                            }
                            _ => (),
                        },
                        Err(_) => (),
                    }
                }
            }
        }
        aliens_index_to_destroy.sort_by(|a, b| b.cmp(a));
        for idx in aliens_index_to_destroy {
            let _ = self.aliens[idx].destroy();
            self.aliens.remove(idx);
        }
        coordinate_if_collided
    }

    pub fn destroy(&mut self, coordinate: Coordinate) {
        for i in 0..self.aliens.len() {
            if self.aliens[i].contains(coordinate) {
                self.aliens[i].destroy();
                let _ = self.aliens.remove(i);
                break;
            }
        }
    }
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
        let mut pointer = space_between; // Start the pointer at space_between
        while pointer + alien_width <= eng.width {
            // Ensure we stay within bounds
            let position = vec![
                (pointer, row),
                (pointer + 1, row),
                (pointer + 2, row),
                (pointer, row + 1),
                (pointer + 1, row + 1),
                (pointer + 2, row + 1),
            ];
            collector.push(Alien::new(engine.clone(), position, velocity)?);
            pointer += alien_width + space_between; // Update pointer to next position
                                                    //pointer += space_between;
        }
    }
    Ok(collector)
}

pub fn farthest_right_alien(aliens: &[Alien]) -> usize {
    let mut res_index = 0;
    let mut memo = aliens[0].far_right();
    for i in 0..aliens.len() {
        let alien_right = aliens[i].far_right();
        if alien_right > memo {
            memo = alien_right;
            res_index = i;
        }
    }
    res_index
}

pub fn farthest_left_alien(aliens: &[Alien]) -> usize {
    let mut res_index = 0;
    let mut memo = aliens[0].far_left();
    for i in 0..aliens.len() {
        let alien_left = aliens[i].far_left();
        if alien_left < memo {
            memo = alien_left;
            res_index = i;
        }
    }
    res_index
}

#[derive(Debug, Clone)]
pub struct Alien {
    sprite: Sprite,
    direction: Direction,
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
            direction: Direction::Right,
            sprite: Sprite::new(engine, position, velocity)?,
        })
    }

    pub fn far_right(&self) -> usize {
        self.sprite.far_right()
    }

    pub fn far_left(&self) -> usize {
        self.sprite.far_left()
    }

    pub fn invert_direction(&mut self) {
        match self.direction {
            Direction::Left => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Left,
        }
    }

    pub fn spawn(&mut self) -> Result<(), Error> {
        let _ = self.sprite.spawn()?;
        Ok(())
    }

    pub fn step(&mut self, step: i32) -> Option<Coordinate> {
        if self.sprite.is_destroyed() {
            return None;
        }
        match self.direction {
            Direction::Left => {
                if self.sprite.far_left() == 0 {
                    return None;
                }
                return match self.sprite.move_relative_x(step) {
                    Ok(state) => match state {
                        State::Collided(coordinate) => {
                            self.sprite.destroy();
                            Some(coordinate)
                        }
                        _ => None,
                    },
                    Err(_) => None,
                };
            }
            Direction::Right => {
                if self.sprite.far_right() == self.width - 1 {
                    self.direction = Direction::Left;
                    return None;
                }
                return match self.sprite.move_relative_x(step) {
                    Ok(state) => match state {
                        State::Collided(coordinate) => {
                            self.sprite.destroy();
                            Some(coordinate)
                        }
                        _ => None,
                    },
                    Err(_) => None,
                };
            }
        }
    }

    pub fn destroy(&mut self) {
        let _ = self.sprite.destroy();
    }
}

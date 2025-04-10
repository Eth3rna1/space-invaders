use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::entities::{Aliens, Bullet, Shooter};
use crate::errors::{Error, ErrorKind};
use crate::listener::get_key;
use crate::utils;
use crate::engine::sprite::State;
use crate::{
    ALIEN_COUNT, ALIEN_STEP_PER_DELTA, BACKGROUND_CHAR, BULLET_STEP_PER_DELTA, PIXEL_CHAR,
    SHOOTER_STEP_PER_DELTA,
};

use std::cell::RefCell;
use std::rc::Rc;


#[derive(Clone, Debug)]
pub struct SpaceInvaders {
    pub(crate) engine: Rc<RefCell<Engine>>,
    pub(crate) shooter: Shooter,
    pub(crate) bullets: Vec<Bullet>,
    pub(crate) aliens: Aliens,
    pub(crate) key: Option<String>,
}

impl SpaceInvaders {
    pub fn new(dimensions: (usize, usize)) -> Result<Self, Error> {
        let engine = Engine::new(dimensions).as_rc();
        let shooter: Shooter = {
            let position: Vec<Coordinate> = {
                let eng = engine.borrow();
                vec![
                    (eng.width / 2, eng.height - (eng.height / 7)),
                    (eng.width / 2 + 1, eng.height - (eng.height / 7)),
                    (eng.width / 2 - 1, eng.height - (eng.height / 7)),
                    (eng.width / 2, eng.height - (eng.height / 7) - 1),
                ]
            };
            Shooter::new(engine.clone(), position, SHOOTER_STEP_PER_DELTA)?
        };
        let aliens: Aliens = Aliens::new(engine.clone(), ALIEN_COUNT, ALIEN_STEP_PER_DELTA)?;
        Ok(Self {
            shooter,
            engine,
            aliens,
            bullets: Vec::new(),
            key: None,
        })
    }

    pub fn set_up(&mut self) {
        utils::clear();
        self.aliens.spawn();
        self.shooter.spawn();
    }

    pub fn handle_input(&mut self) {
        self.key = get_key();
    }

    pub fn update(&mut self, delta_time: f32) {
        {
            // checking key pressed and spawning bullets if a space is pressed
            if let Some(key) = &self.key {
                match key.as_str() {
                    " " => {
                        // spawning a bullet
                        if let Ok(mut bullet) = Bullet::new(
                            self.engine.clone(),
                            self.shooter.head(),
                            BULLET_STEP_PER_DELTA,
                        ) {
                            if let Ok(_) = bullet.spawn() {
                                self.bullets.push(bullet);
                            }
                        }
                    }
                    "left" | "right" => {
                        let _ = self.shooter.step(&key, delta_time);
                    }
                    _ => {}
                }
            }
        }
        {
            // moving all bullets
            let mut bullets_to_destroy: Vec<usize> = Vec::new();
            for i in 0..self.bullets.len() {
                match self.bullets[i].step(delta_time) {
                    Ok(state) => match state {
                        State::Collided(coordinate) => {
                            if self.aliens.find_and_destroy(coordinate) {
                                self.bullets[i].destroy();
                                bullets_to_destroy.push(i);
                            }
                        }
                        _ => {},
                    },
                    Err(error) => match error.kind() {
                        ErrorKind::OutOfBounds => {
                            self.bullets[i].destroy();
                            bullets_to_destroy.push(i);
                        },
                        _ => {}
                    }
                }
            }
            // removing bullets from the vector
            for idx in bullets_to_destroy.iter() {
                self.bullets.remove(*idx);
            }
        }
        {
            // moving the aliens
            self.aliens.step(delta_time);
        }
        self.key = None;
    }

    pub fn draw(&mut self) {
        print!(
            "{}",
            self.engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR)
        );
        utils::refresh();
    }

    pub fn game_over(&self) -> bool {
        self.aliens.is_empty()
    }
}

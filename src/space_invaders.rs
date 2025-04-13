use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::entities::{
    Bullet, Shooter, Speedster, {farthest_left_alien, farthest_right_alien, Aliens},
};
use crate::errors::{Error, ErrorKind};
use crate::listener::get_key;
use crate::utils;
use crate::{
    ALIEN_COUNT, ALIEN_STEP_PER_DELTA, BACKGROUND_CHAR, BULLET_STEP_PER_DELTA, PIXEL_CHAR,
    SHOOTER_STEP_PER_DELTA, SPEEDSTER_STEP_PER_DELTA,
};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SpaceInvaders {
    pub(crate) aliens: Aliens,
    pub(crate) game_over: bool,
    pub(crate) shooter: Shooter,
    pub(crate) key: Option<String>,
    pub(crate) bullets: Vec<Bullet>,
    pub(crate) speedster: Speedster,
    pub(crate) engine: Rc<RefCell<Engine>>,
    won: bool,
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
        let speedster: Speedster = Speedster::new(engine.clone(), SPEEDSTER_STEP_PER_DELTA)?;
        Ok(Self {
            engine,
            aliens,
            shooter,
            key: None,
            speedster,
            won: false,
            game_over: false,
            bullets: Vec::new(),
        })
    }

    pub fn set_up(&mut self) {
        utils::clear();
        self.aliens.spawn();
        self.shooter.spawn();
    }

    /// Memoizes the key pressed in an instant
    pub fn handle_input(&mut self) {
        self.key = get_key();
    }

    /// checking key pressed and spawning bullets if a space is pressed
    fn _update_upon_key_press(&mut self, delta_time: f32) {
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

    /// moving the aliens
    pub fn _move_aliens(&mut self, delta_time: f32) {
        if let Some(coordinate) = self.aliens.step(delta_time) {
            // perhaps hits a bullet
            let mut bullets_to_destroy: Vec<usize> = Vec::new();
            for i in 0..self.bullets.len() {
                let mut bullet = &mut self.bullets[i];
                if !bullet.is_alien_bullet() && bullet.contains(coordinate) {
                    bullet.destroy();
                    bullets_to_destroy.push(i);
                }
            }
            for i in bullets_to_destroy {
                let _ = self.bullets.remove(i);
            }
        }
    }

    /// moving all bullets
    pub fn _move_bullets(&mut self, delta_time: f32) {
        let mut bullets_to_destroy: Vec<usize> = Vec::new();
        for i in 0..self.bullets.len() {
            if let Some(coordinate) = self.bullets[i].step(delta_time) {
                // coordinate of the sprite it collided with
                if self.aliens.find_and_destroy(coordinate) {
                    self.bullets[i].destroy();
                    bullets_to_destroy.push(i);
                } else if self.shooter.contains(coordinate) {
                    // an alien bullet that hit the player
                    self.game_over = true;
                    return;
                } else if self.speedster.contains(coordinate) {
                    // speedster vs the player in the end game
                    self.game_over = true;
                    self.won = true;
                }
            }
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        {
            // sends the key to the shooter or spawns new bullets
            self._update_upon_key_press(delta_time);
        }
        {
            // moves aliens, taking into account collisions with bullets,
            // if collides with a bullet, both the alien and bullet get destroyed
            self._move_aliens(delta_time);
        }
        {
            // moves bullets, taking into account collisions with other sprites.
            // If collides with other sprites, both the bullet and sprite gets
            // destroyed.
            // If the bullet collides with the speedster, the game is automatically
            // a win for the player.
            self._move_bullets(delta_time);
        }
    }

    pub fn draw(&mut self) {
        print!(
            "{}",
            self.engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR)
        );
        utils::refresh();
    }

    pub fn game_over(&self) -> bool {
        //self.aliens.is_empty() && self.speedster.is_dead() || self.game_over
        self.aliens.is_empty() || self.game_over
    }
}

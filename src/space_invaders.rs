use crate::engine::{
    sprite::{self, Sprite, State},
    Coordinate, Engine,
};
use crate::entities::{Aliens, Bullet, Shooter, Speedster, SpeedsterBullet};
use crate::errors::{Error, ErrorKind};
use crate::listener::{get_key, key_pressed};
use crate::utils;
use crate::{
    ALIEN_COUNT,
    ALIEN_STEP_PER_DELTA,
    BACKGROUND_CHAR,
    BULLET_STEP_PER_DELTA,
    DELTA_TIME,
    PIXEL_CHAR,
    PLANE_DIMENSIONS, // (WIDTH, HEIGHT)
    SHOOTER_STEP_PER_DELTA,
    SPEEDSTER_BULLET_PER_DELTA,
    SPEEDSTER_STEP_PER_DELTA,
};

use std::cell::RefCell;
use std::rc::Rc;

struct SpaceInvaders {
    key: Option<String>,
    engine: Rc<RefCell<Engine>>,
    aliens: Aliens,
    shooter: Shooter,
    bullets: Vec<Bullet>,
    speedster: Speedster,
    //speedster_bullets: Vec<SpeedsterBullet>,
}

impl SpaceInvaders {
    fn new() -> Result<Self, Error> {
        //let PLANE_DIMENSIONS: (usize, usize) = {
        //    let (x, y) = terminal::size().unwrap();
        //    (x as usize - 2, y as usize - 3)
        //};
        let engine = Engine::new(PLANE_DIMENSIONS).as_rc();
        Ok(Self {
            key: None,
            aliens: Aliens::new(engine.clone(), ALIEN_COUNT, ALIEN_STEP_PER_DELTA)?,
            shooter: Shooter::new(engine.clone(), SHOOTER_STEP_PER_DELTA)?,
            speedster: Speedster::new(engine.clone(), SPEEDSTER_STEP_PER_DELTA)?,
            //speedster_bullets: Vec::new(),
            engine,
            bullets: Vec::new(),
        })
    }

    fn set_up(&mut self) {
        self.shooter.spawn();
        self.aliens.spawn();
        //self.speedster.spawn();
    }

    fn handle_input(&mut self) {
        self.key = get_key();
    }

    /// I want to procure moving objects first before handling user input
    fn update(&mut self) -> Result<(), String> {
        let mut result: Result<(), String> = Ok(());
        {
            // moving bullets
            //for bullet in self.bullets.iter_mut() {
            let mut collided_coordinate_if_any: Option<Coordinate> = None;
            let bullets_len = self.bullets.len();
            for i in 0..bullets_len {
                let mut bullet = &mut self.bullets[i];
                let _result = bullet.step();
                if let Err(error) = _result {
                    //result = Err(error.diagnosis());
                    // I think I want to return early
                    result.clone()?;
                } else if let Ok(State::Collided(coordinate)) = _result {
                    // The bullet hit something
                    collided_coordinate_if_any = Some(coordinate);
                    bullet.destroy();
                    break;
                }
            }
            // checking for bullet collisions
            if let Some(coordinate) = collided_coordinate_if_any {
                let _ = utils::check_collision_and_destroy(
                    coordinate,
                    &mut self.aliens,
                    &mut self.speedster,
                    &mut self.bullets,
                );
            }
        }
        {
            // moving player
            if let Some(ref key) = self.key {
                if let Err(error) = self.shooter.step(key) {
                    return Err(error.diagnosis());
                }
            }
        }
        {
            // moving aliens
            let result = self.aliens.step();
            if let Ok(State::Collided(coordinate)) = result {
                for bullet in self.bullets.iter_mut() {
                    if bullet.contains(coordinate) {
                        bullet.destroy();
                        break;
                    }
                }
            } else if result.is_err() {
                return Err(result.unwrap_err().diagnosis());
            }
        }
        {
            // moving speedster
            //if self.speedster.is_destroyed() {
            //    self.speedster.respawn();
            //}
            //let result = self.speedster.step();
            //if let Ok(State::Collided(coordinate)) = result {
            //    for bullet in &mut self.bullets {
            //        if bullet.contains(coordinate) {
            //            bullet.destroy();
            //            self.speedster.destroy();
            //            break;
            //        }
            //    }
            //} else if result.is_err() {
            //    return Err(result.unwrap_err().diagnosis());
            //}
        }
        {
            //// spawning speedster bullets
            //match SpeedsterBullet::new(
            //    self.engine.clone(),
            //    vec![self.speedster.tail()],
            //    SPEEDSTER_BULLET_PER_DELTA,
            //) {
            //    Ok(sprite) => self.speedster_bullets.push(sprite),
            //    Err(error) => return Err(error.diagnosis()),
            //}
        }
        {
            // moving speedster bullets
            //for bullet in self.speedster_bullets.iter_mut() {
            //    if let Err(error) = bullet.step() {
            //        return Err(error.diagnosis());
            //    }
            //}
        }
        {
            // checking for new bullet
            if self.key == Some(" ".to_string()) {
                let position = self.shooter.head();
                match Bullet::new(self.engine.clone(), vec![position], BULLET_STEP_PER_DELTA) {
                    Ok(mut bullet) => {
                        let _ = bullet.spawn();
                        self.bullets.push(bullet);
                    }
                    Err(err) => return Err(err.diagnosis()),
                }
            }
        }
        result
    }

    fn output(&self) {
        print!(
            "\n{}",
            self.engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR)
        );
    }

    fn over(&self) -> bool {
        if let Some(ref k) = get_key() {
            return k == "esc";
        }
        false
    }
}

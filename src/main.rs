#![allow(warnings)]
/*
    Remaking the game Space Invaders in ASCII

To-Do:
    use the signal_hook crate
*/
mod engine;
mod entities;
mod errors;
mod listener;
mod utils;

use crossterm::terminal;
use engine::{
    sprite::{self, Sprite, State},
    Coordinate, Engine,
};
use entities::{Aliens, Bullet, Shooter, Speedster};
use errors::{Error, ErrorKind};
use listener::{get_key, key_pressed};
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

const DELTA_TIME: f64 = 0.07;
const ALIEN_COUNT: usize = 6;
//const PLANE_DIMENSIONS: Coordinate = (100, 25); // (WIDTH, HEIGHT)
const PLANE_DIMENSIONS: Coordinate = (100, 20); // (WIDTH, HEIGHT)

//const PIXEL_CHAR: char = '█';
//const PIXEL_CHAR: char = '▀';
const PIXEL_CHAR: char = '⨊';
const BACKGROUND_CHAR: char = '.';

const ALIEN_STEP_PER_DELTA: usize = 1;
const BULLET_STEP_PER_DELTA: usize = 2;
const SHOOTER_STEP_PER_DELTA: usize = 3;
const SPEEDSTER_STEP_PER_DELTA: usize = 4;

struct SpaceInvaders {
    key: Option<String>,
    engine: Rc<RefCell<Engine>>,
    aliens: Aliens,
    shooter: Shooter,
    bullets: Vec<Bullet>,
    speedster: Speedster,
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
            engine,
            bullets: Vec::new(),
        })
    }

    fn set_up(&mut self) {
        self.shooter.spawn();
        self.aliens.spawn();
        self.speedster.spawn();
    }

    fn handle_input(&mut self) {
        self.key = get_key();
    }

    fn update(&mut self) -> Result<(), String> {
        let mut result: Result<(), String> = Ok(());
        {
            // checking for new bullet
            if self.key == Some(" ".to_string()) {
                let position = self.shooter.head();
                match Bullet::new(self.engine.clone(), vec![position], BULLET_STEP_PER_DELTA) {
                    Ok(mut bullet) => {
                        let _ = bullet.spawn();
                        self.bullets.push(bullet);
                    }
                    Err(err) => result = Err(err.diagnosis()),
                }
            }
        }
        {
            // moving bullets
            for bullet in self.bullets.iter_mut() {
                let _result = bullet.step();
                if let Err(error) = _result {
                    result = Err(error.diagnosis());
                } else if let Ok(State::Collided(coordinate)) = _result {
                    // The bullet hit something
                    if self.speedster.contains(coordinate) {
                        self.speedster.destroy();
                        bullet.destroy();
                    }
                    for alien in self.aliens.iter_mut() {
                        if alien.contains(coordinate) {
                            alien.destroy();
                            bullet.destroy();
                            break;
                        }
                    }
                }
            }
        }
        {
            // moving player
            if let Some(ref key) = self.key {
                if let Err(error) = self.shooter.step(key) {
                    result = Err(error.diagnosis());
                }
            }
        }
        {
            // moving aliens
            if let Ok(State::Collided(coordinate)) = self.aliens.step() {
                for bullet in self.bullets.iter_mut() {
                    if bullet.contains(coordinate) {

                        bullet.destroy();
                        break;
                    }
                }
            }
        }
        {
            // moving speedster
            if self.speedster.is_destroyed() {
                self.speedster.respawn();
            }
            if let Ok(State::Collided(coordinate)) = self.speedster.step() {
                for bullet in &mut self.bullets {
                    if bullet.contains(coordinate) {
                        bullet.destroy();
                        self.speedster.destroy();
                        break;
                    }
                }
                //for bullet
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

fn main() -> Result<(), Error> {
    //terminal::enable_raw_mode();
    utils::clear();
    let mut banner: String = "Welcome to Space Invaders".to_string();
    let mut game = SpaceInvaders::new()?;
    game.set_up();
    loop {
        game.handle_input();
        if let Err(msg) = game.update() {
            banner = msg.to_string();
        }
        println!("{}{}", banner, " ".repeat(15));
        game.output();
        if game.over() {
            break;
        }
        utils::sleep(DELTA_TIME);
        utils::refresh();
    }
    //terminal::disable_raw_mode();
    Ok(())
}

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
    sprite::{self, Sprite},
    Coordinate, Engine,
};
use entities::{Aliens, Shooter};
use errors::{Error, ErrorKind};
use listener::{get_key, key_pressed};
use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

const DELTA_TIME: f64 = 0.05;
const ALIEN_COUNT: usize = 7;
const PIXEL_CHAR: char = '#';
const BACKGROUND_CHAR: char = '.';
const PLANE_DIMENSIONS: Coordinate = (100, 25); // (WIDTH, HEIGHT)
const SHOOTER_STEP_PER_DELTA: usize = 1;

fn spawn_aliens(engine: Rc<RefCell<Engine>>) -> Result<Sprite, Error> {
    todo!()
}

fn spawn_shooter(engine: Rc<RefCell<Engine>>) -> Result<Sprite, Error> {
    let mut position: Vec<Coordinate> = {
        let mut eng = engine.borrow();
        vec![
            (eng.width / 2, eng.length - (eng.length / 7)),
            (eng.width / 2 - 1, eng.length - (eng.length / 7)),
            (eng.width / 2 + 1, eng.length - (eng.length / 7)),
            (eng.width / 2, eng.length - (eng.length / 7) - 1),
        ]
    };
    let mut shooter = Sprite::new(engine, position, 3)?;
    shooter.spawn();
    Ok(shooter)
}

fn spawn_bullet(engine: Rc<RefCell<Engine>>, position: Coordinate) -> Result<Sprite, Error> {
    let mut bullet = Sprite::new(engine, vec![position], 3)?;
    bullet.spawn();
    Ok(bullet)
}

struct SpaceInvaders {
    key: Option<String>,
    engine: Rc<RefCell<Engine>>,
    //alines: Aliens,
    shooter: Shooter,
}

impl SpaceInvaders {
    fn new() -> Result<Self, Error> {
        let engine = Engine::new(PLANE_DIMENSIONS).as_rc();
        Ok(Self {
            key: None,
            shooter: Shooter::new(engine.clone(), SHOOTER_STEP_PER_DELTA)?,
            engine,
        })
    }

    fn set_up(&mut self) {
        self.shooter.spawn();
    }

    fn handle_input(&mut self) {
        self.key = get_key();
    }

    fn update(&mut self) -> Result<(), Error> {
        if let Some(ref key) = self.key {
            self.shooter.step(key)?;
        }
        Ok(())
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
    let mut banner: String = "Welcome to SpaceInvaders".to_string();
    let mut game = SpaceInvaders::new()?;
    game.set_up();
    loop {
        game.handle_input();
        if let Err(msg) = game.update() {
            banner = msg.to_string();
        }
        game.update();
        println!("{}{}", banner, " ".repeat(20));
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

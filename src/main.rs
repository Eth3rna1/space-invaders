#![allow(warnings)]
/*
    Remaking the game Space Invaders in ASCII
*/
mod engine;
mod entities;
mod errors;
mod listener;
mod space_invaders;
mod utils;

use crossterm::terminal;
use engine::{
    sprite::{self, Sprite, State},
    Coordinate, Engine,
};
use errors::{Error, ErrorKind};
use listener::{get_key, key_pressed};
use space_invaders::SpaceInvaders;

use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Instant;

//static mut DELTA_TIME: f64 = 0.07;
//static mut DELTA_TIME: f64 = 0.5;
pub const ALIEN_COUNT: usize = 6;
//pub const ALIEN_COUNT: usize = 8;
const PLANE_DIMENSIONS: Coordinate = (100, 25); // (WIDTH, HEIGHT)

//const PIXEL_CHAR: char = '█';
//const PIXEL_CHAR: char = '▀';
pub const PIXEL_CHAR: char = '⨊';
pub const BACKGROUND_CHAR: char = '.';

pub const ALIEN_STEP_PER_DELTA: f32 = 15.0;
pub const BULLET_STEP_PER_DELTA: f32 = 9.0;
//pub const SHOOTER_STEP_PER_DELTA: f64 = 3.0;
pub const SHOOTER_STEP_PER_DELTA: f32 = 90.0;
pub const SPEEDSTER_STEP_PER_DELTA: f32 = 2.0;
pub const SPEEDSTER_BULLET_PER_DELTA: f32 = 2.0;

fn main() -> Result<(), Error> {
    let mut game = SpaceInvaders::new(PLANE_DIMENSIONS)?;
    game.set_up();
    let mut delta_time: f32 = 1.0;
    let game_timer = Instant::now();
    loop {
        let start = Instant::now();
        game.handle_input();
        game.update(delta_time);
        game.draw();
        if game.game_over() {
            break;
        }
        delta_time = (Instant::now() - start).as_secs_f32();
    }
    println!(
        "You finished the game in: {:?}\n",
        Instant::now() - game_timer
    );
    Ok(())
}

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

pub const ALIEN_COL_COUNT: usize = 6;
const PLANE_DIMENSIONS: Coordinate = (100, 25); // (WIDTH, HEIGHT)

//const PIXEL_CHAR: char = '█';
//const PIXEL_CHAR: char = '▀';
pub const PIXEL_CHAR: char = '⨊';
pub const BACKGROUND_CHAR: char = '.';

pub const OBSTACLE_WAIT_TIME: f32 = 2.0; // seconds
pub const OBSTACLE_SPEED: f32 = 80.0;

pub const ALIEN_STEP_PER_DELTA: f32 = 15.0;
pub const BULLET_STEP_PER_DELTA: f32 = 9.0;
pub const SHOOTER_STEP_PER_DELTA: f32 = 90.0;
pub const SPEEDSTER_STEP_PER_DELTA: f32 = 90.0;
pub const SPEEDSTER_BULLET_PER_DELTA: f32 = 2.0;

fn main() -> Result<(), Error> {
    let mut game = SpaceInvaders::new(PLANE_DIMENSIONS)?;
    game.set_up();
    let mut delta_time: f32 = 0.0;
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
    let msg = match game.won() {
        true => "You won :)",
        false => "You lost :(",
    };
    println!("{:^100}\n{:^100}", ' ', msg);
    let time_result = format!(
        "You finished the game in: {:?}",
        Instant::now() - game_timer
    );
    println!("{:^100}\n{:^100}\n", ' ', time_result);
    Ok(())
}

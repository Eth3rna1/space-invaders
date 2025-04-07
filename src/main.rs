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

pub const ALIEN_STEP_PER_DELTA: f64 = 15.0;
pub const BULLET_STEP_PER_DELTA: f64 = 9.0;
//pub const SHOOTER_STEP_PER_DELTA: f64 = 3.0;
pub const SHOOTER_STEP_PER_DELTA: f64 = 90.0;
pub const SPEEDSTER_STEP_PER_DELTA: f64 = 2.0;
pub const SPEEDSTER_BULLET_PER_DELTA: f64 = 2.0;

fn main() -> Result<(), Error> {
    let mut game = SpaceInvaders::new(PLANE_DIMENSIONS)?;
    game.set_up();
    let mut delta_time: f64 = 1.0;
    loop {
        let start = Instant::now();
        game.handle_input();
        //game.update(unsafe { DELTA_TIME });
        game.update(delta_time);
        game.draw();
        if game.game_over() {
            break;
        }
        delta_time = (Instant::now() - start).as_secs_f64();
        //unsafe {
        //    utils::sleep(DELTA_TIME); // forcing delta time
        //}
    }
    Ok(())
}

//fn main() -> Result<(), Error> {
//    utils::clear();
//    let mut engine = Engine::new(PLANE_DIMENSIONS).as_rc();
//    let mut square = Sprite::new(
//        engine.clone(),
//        // start at the far left side and moving the box right
//        vec![
//            //(0, PLANE_DIMENSIONS.1 / 2),
//            //(1, PLANE_DIMENSIONS.1 / 2),
//            //(0, PLANE_DIMENSIONS.1 / 2 - 1),
//            //(1, PLANE_DIMENSIONS.1 / 2 - 1),
//            (PLANE_DIMENSIONS.0 - 1, PLANE_DIMENSIONS.1 / 2),
//            (PLANE_DIMENSIONS.0 - 2, PLANE_DIMENSIONS.1 / 2),
//            (PLANE_DIMENSIONS.0 - 1, PLANE_DIMENSIONS.1 / 2 - 1),
//            (PLANE_DIMENSIONS.0 - 2, PLANE_DIMENSIONS.1 / 2 - 1),
//        ],
//        //1.0,
//        2.0
//    )?;
//    square.spawn()?;
//    //let mut delta_time: f64 = 1.0;
//    loop {
//        //                                      PROBLEM: SQUARE FLIES BY
//        let start = Instant::now();
//        //println!("{:?}", square.coordinates);
//        //let ex = square.exact_x();
//        //let offset = square.velocity() * delta_time;
//        //let step = dbg!((ex + offset) as usize - ex as usize);
//        //dbg!(&ex, &offset, &delta_time);
//        //if step == 0 {
//        //    square.offset_exact_x(offset);
//        //    //continue
//        //}
//        //square.move_relative_x(step as i32)?;
//        square.move_relative_x(-1)?;
//        println!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
//        utils::sleep(0.05);
//        utils::refresh();
//        //utils::clear();
//        //delta_time = (Instant::now() - start).as_secs_f64();
//    }
//   Ok(())
//}

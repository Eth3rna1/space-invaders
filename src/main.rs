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

static mut DELTA_TIME: f64 = 0.07;
//static mut DELTA_TIME: f64 = 0.5;
const ALIEN_COUNT: usize = 6;
const PLANE_DIMENSIONS: Coordinate = (100, 25); // (WIDTH, HEIGHT)

//const PIXEL_CHAR: char = '█';
//const PIXEL_CHAR: char = '▀';
const PIXEL_CHAR: char = '⨊';
const BACKGROUND_CHAR: char = '.';

const ALIEN_STEP_PER_DELTA: f64 = 1.0;
const BULLET_STEP_PER_DELTA: f64 = 2.0;
const SHOOTER_STEP_PER_DELTA: f64 = 3.0;
const SPEEDSTER_STEP_PER_DELTA: f64 = 2.0;
const SPEEDSTER_BULLET_PER_DELTA: f64 = 2.0;

//fn main() -> Result<(), Error> {
//    //terminal::enable_raw_mode();
//    utils::clear();
//    let mut banner: String = "Welcome to Space Invaders".to_string();
//    let mut game = SpaceInvaders::new()?;
//    game.set_up();
//    loop {
//        game.handle_input();
//        if let Err(msg) = game.update() {
//            banner = msg.to_string();
//        }
//        println!("{}{}", banner, " ".repeat(15));
//        game.output();
//        if game.over() {
//            break;
//        }
//        utils::sleep(DELTA_TIME);
//        utils::refresh();
//    }
//    //terminal::disable_raw_mode();
//    Ok(())
//}

fn left_to_right(
    plane_dimensions: (usize, usize),
    delta_time: f64,
    velocity: f64,
) -> Result<(), Error> {
    utils::clear();
    let engine = Engine::new(plane_dimensions).as_rc();
    let height = { engine.borrow().height };
    let (width, height) = {
        let eng = engine.borrow();
        (eng.width, eng.height)
    };
    let position = {
        vec![
            (0, height / 2),
            (0, height / 2 - 1),
            (1, height / 2),
            (1, height / 2 - 1),
        ]
    };
    let mut square = Sprite::new(engine.clone(), position, velocity)?;
    square.spawn()?;
    loop {
        match square.move_right() {
            Ok(_) => (),
            Err(err) => {
                print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR)); // outputs the
                                                                                    // last state of the game
                break;
            }
        }
        print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
        utils::sleep(delta_time);
        utils::refresh();
        //utils::clear();
    }
    Ok(())
}

fn right_to_left(
    plane_dimensions: (usize, usize),
    delta_time: f64,
    velocity: f64,
) -> Result<(), Error> {
    utils::clear();
    let engine = Engine::new(plane_dimensions).as_rc();
    let height = { engine.borrow().height };
    let (width, height) = {
        let eng = engine.borrow();
        (eng.width, eng.height)
    };
    let position = {
        vec![
            (width - 1, height / 2),
            (width - 1, height / 2 - 1),
            (width - 2, height / 2),
            (width - 2, height / 2 - 1),
        ]
    };
    let mut square = Sprite::new(engine.clone(), position, velocity)?;
    square.spawn()?;
    loop {
        match square.move_left() {
            Ok(_) => (),
            Err(err) => {
                print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
                break;
            }
        }
        print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
        utils::sleep(delta_time);
        utils::refresh();
    }
    Ok(())
}

fn top_to_bottom(
    plane_dimensions: (usize, usize),
    delta_time: f64,
    velocity: f64,
) -> Result<(), Error> {
    utils::clear();
    let engine = Engine::new(plane_dimensions).as_rc();
    let height = { engine.borrow().height };
    let (width, height) = {
        let eng = engine.borrow();
        (eng.width, eng.height)
    };
    let position = {
        vec![
            (width / 2, 0),
            (width / 2 - 1, 0),
            (width / 2, 1),
            (width / 2 - 1, 1),
        ]
    };
    let mut square = Sprite::new(engine.clone(), position, velocity)?;
    square.spawn()?;
    loop {
        match square.move_down() {
            Ok(_) => (),
            Err(err) => {
                //eprintln!("{:#?}", square);
                print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
                break;
            }
        }
        print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
        utils::sleep(delta_time);
        utils::refresh();
        //utils::clear();
    }
    Ok(())
}

fn bottom_to_top(
    plane_dimensions: (usize, usize),
    delta_time: f64,
    velocity: f64,
) -> Result<(), Error> {
    utils::clear();
    let engine = Engine::new(plane_dimensions).as_rc();
    let height = { engine.borrow().height };
    let (width, height) = {
        let eng = engine.borrow();
        (eng.width, eng.height)
    };
    let position = {
        vec![
            (width / 2, height - 1),
            (width / 2 - 1, height - 1),
            (width / 2, height - 2),
            (width / 2 - 1, height - 2),
        ]
    };
    let mut square = Sprite::new(engine.clone(), position, velocity)?;
    square.spawn()?;
    loop {
        match square.move_up() {
            Ok(_) => (),
            Err(err) => {
                println!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
                break;
            }
        }
        print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
        utils::sleep(delta_time);
        utils::refresh();
        //utils::clear();
    }
    Ok(())
}

fn left_to_right_with_obstacle(p: (usize, usize), d: f64, v: f64) -> Result<(), Error> {
    utils::clear();
    let engine = Engine::new(p).as_rc();
    let (width, height) = {
        let eng = engine.borrow();
        (eng.width, eng.height)
    };
    let position = {
        vec![
            (0, height / 2),
            (0, height / 2 - 1),
            (1, height / 2),
            (1, height / 2 - 1),
        ]
    };
    let mut square = Sprite::new(engine.clone(), position, v)?;
    square.spawn()?;
    {
        // spawning an obstacle
        let mut eng = engine.borrow_mut();
        eng.spawn((width - (width / 5), height / 2));
        //eng.spawn((width - (width / 5) + 1, height / 2));
        //eng.spawn((width - (width / 5) + 2, height / 2));
        //eng.spawn((width - (width / 5) + 3, height / 2));
        //eng.spawn((width - (width / 5) + 4, height / 2));
    }
    loop {
        match square.move_right() {
            Ok(state) => {
                println!("{:?}", state);
                match state {
                    State::Collided(c) => {
                        println!("Coordinate Collided With: {:?}", c);
                        print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
                        break;
                    }
                    _ => (),
                }
            }
            Err(err) => {
                print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
                break;
            }
        }
        print!("{}", engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR));
        utils::sleep(d);
        utils::refresh();
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    //let plane_dimensions = (100, 10);
    //let plane_dimensions = (10, 25);
    let plane_dimensions = (50, 20);
    let delta_time = unsafe { DELTA_TIME };
    let velocity = 15.9231231f64;
    left_to_right(plane_dimensions, delta_time, velocity)?;
    top_to_bottom(plane_dimensions, delta_time, velocity)?;
    right_to_left(plane_dimensions, delta_time, velocity)?;
    bottom_to_top(plane_dimensions, delta_time, velocity)?;
    left_to_right_with_obstacle(plane_dimensions, delta_time, velocity)?;
    Ok(())
}

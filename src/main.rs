/*
    Remaking the game Space Invaders in ASCII
*/
mod constants;
mod engine;
mod math;
mod render;
mod tool;
//mod tests;
mod errors;

use crossterm::terminal; // contains the size() function to measure terminal
use engine::{Engine, sprite, Coordinate};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut frames: Vec<String> = Vec::new();
    let dimensions = (100usize, 30usize);
    let mut engine = Rc::new(RefCell::new(Engine::new(dimensions)));
    let square_coordinates: [Coordinate; 4] = [
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1)
    ];
    let mut square: sprite::Sprite = sprite::Sprite::new(Rc::clone(&engine), square_coordinates).unwrap();
    square.spawn();
    frames.push(engine.borrow().output());
    square.move_down();
    frames.push(engine.borrow().output());
    tool::clear();
    use std::thread;
    use std::time::Duration;
    for frame in frames.iter() {
        println!("{}", frame);
        thread::sleep(Duration::from_secs(1));
        tool::refresh();
    }
    //tool::clear();
}

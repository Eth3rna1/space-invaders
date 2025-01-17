/*
    Remaking the game Space Invaders in ASCII
*/
mod constants;
mod engine;
mod math;
mod render;
mod tool;
mod tests;
mod errors;

use crossterm::terminal; // contains the size() function to measure terminal
use engine::{Engine, sprite, Coordinate};

fn main() {
    let mut frames: Vec<String> = Vec::new();
    let dimensions = (100usize, 30usize);
    let mut eng = Engine::new(dimensions);
    let mut square: &[Coordinate] = &[
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1)
    ];
    sprite::spawn_sprite(&mut eng, &square);
    frames.push(eng.output());
    //println!("{}", eng.output());
    tool::clear();
    use std::thread;
    use std::time::Duration;
    for frame in frames.iter() {
        println!("{}", frame);
        thread::sleep(Duration::from_secs(1));
        tool::refresh();
    }
}

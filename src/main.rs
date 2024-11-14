/*
    Remaking the game Space Invaders in ASCII
*/
mod constants;
mod engine;
mod math;
mod render;
mod tool;
mod tests;
use crossterm::terminal; // contains the size() function to measure terminal
use engine::{pool, Engine};
use render::Render;


fn main() {
    //tests::zigzag();
    //tests::line(); // not yet implemented
    println!("Compiled Successfully!");
}

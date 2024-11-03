/*
    Remaking the game Space Invaders in ASCII
*/
mod constants;
mod engine;
mod render;
mod tool;
mod tests;
use crossterm::terminal; // contains the size() function to measure terminal
use engine::{pool, Engine};
use render::Render;


fn main() {
    tests::single_zigzag();
}

/*
    Unspecific functions
*/
use crate::engine::{
    sprite::{Sprite, State},
    Coordinate,
};
use crate::entities::{Aliens, Bullet, Speedster};

use std::thread;
use std::time::Duration;

/// Clears the terminal screen
pub(crate) fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

/// Returns the cursor to the top-left of the screen
pub(crate) fn refresh() {
    print!("\x1B[H");
}

/// Delays any thread action
pub(crate) fn sleep(n: f64) {
    thread::sleep(Duration::from_secs_f64(n));
}

pub(crate) fn check_collision_and_destroy(
    coordinate: Coordinate,
    aliens: &mut Aliens,
    speedster: &mut Speedster,
    bullets: &mut [Bullet],
) -> State {
    if speedster.contains(coordinate) {
        return speedster.destroy();
    }
    for alien in aliens.iter_mut() {
        if alien.contains(coordinate) {
            return alien.destroy();
        }
    }
    // check for any bullets that might've gotten stuck
    for bullet in bullets.iter_mut() {
        if bullet.contains(coordinate) {
            return bullet.destroy();
        }
    }
    State::Null
}

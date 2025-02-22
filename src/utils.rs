/*
    Unspecific functions
*/
//pub mod math;
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

pub(crate) fn sleep(n: f64) {
    thread::sleep(Duration::from_secs_f64(n));
}

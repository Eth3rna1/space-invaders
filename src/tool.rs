/*
    Unspecific functions
*/
use std::thread;
use std::time::Duration;

pub(crate) fn clear() {
    // print!("\x1B[2J\x1B[1;1H"); // meant to clear the terminal screen
    // Move cursor back to the top-left before the next iteration
    print!("\x1B[H");
}

pub(crate) fn sleep(n: f64) {
    thread::sleep(Duration::from_secs_f64(n));
}

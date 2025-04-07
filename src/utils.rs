/*
    Unspecific functions
*/
use crate::engine::{
    sprite::{Sprite, State},
    Coordinate,
};

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

pub(crate) fn log<S: AsRef<str> + std::fmt::Debug + std::marker::Send + 'static>(data: S) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::thread::{self, JoinHandle};

    let callback: JoinHandle<std::io::Result<()>> = thread::spawn(move || {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(r"C:\Users\gmend\Rust\lab\spaceinvaders\logs.txt")?;
        writeln!(file, "{:?}", data)?;
        Ok(())
    });
    callback.join();
    Ok(())
}

//! Utility Functions
//!
//! This module provides general-purpose helper functions that are not specific
//! to any one part of the game logic. These utilities assist with terminal
//! manipulation, thread control, and asynchronous logging.
//!
//! # Terminal Helpers
//!
//! - [`clear()`]: Clears the terminal screen using `crossterm`, moving the cursor
//!   to the top-left corner. Ensures a consistent rendering surface for game updates.
//!
//! - [`refresh()`]: Resets the terminal cursor position to `(0, 0)` without clearing
//!   the screen. Useful for updating game state in-place.
//!
//! # Thread Control
//!
//! - [`sleep(n: f64)`]: Pauses the current thread for `n` seconds (fractional allowed).
//!   Uses `Duration::from_secs_f64()` for precision timing, supporting smooth animation steps.
//!
//! # Logging
//!
//! - [`log(data: String)`]:
//!     - Writes log data to a fixed log file asynchronously (`logs.txt`).
//!     - Uses a separate thread to avoid blocking the main game loop.
//!     - Appends to the file, creating it if it doesn't exist.
//!
//! # Notes
//!
//! - All terminal control relies on `crossterm` for cross-platform behavior, but
//!   legacy ANSI escape sequences are provided as commented alternatives.
//!
//! # Example Usage
//!
//! ```rust
//! use crate::utils::{clear, refresh, sleep, log};
//!
//! const FILE_LOG_PATH: String = "./logs.txt".to_string();
//!
//! fn render_frame() {
//!     let mut sprite = ...;
//!     clear();
//!     // draw game entities
//!     refresh();
//!     sleep(0.05);
//!     log(format!("Sprite position: {}", sprite.position()), FILE_LOG_PATH);
//! }
//! ```
use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};

/// Clears the terminal screen
pub fn clear() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))
        .expect("Error trying to clear the screen");
    execute!(stdout(), cursor::MoveTo(0, 0)).expect("Error at moving");
    stdout().flush().expect("Error at flush to stdout");
}
//pub fn clear() {
//    print!("\x1B[2J\x1B[1;1H");
//}

/// Returns the cursor to the top-left of the screen
pub fn refresh() {
    let mut out = stdout();
    out.flush().expect("Error at flusing to stdout");
    execute!(out, cursor::MoveTo(0, 0)).expect("Error at moving");
    out.flush().expect("Error at flusing to stdout");
}
//pub fn refresh() {
//    print!("\x1B[H");
//}

/// Delays any thread action
pub fn sleep(n: f64) {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_secs_f64(n));
}

/// a logging function that writes to a file, helping with debugging.
pub(crate) fn log(data: String, file: String) {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::thread::{self, JoinHandle};

    let callback: JoinHandle<std::io::Result<()>> = thread::spawn(move || {
        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file)
        {
            let _ = writeln!(file, "{}", data);
        }
        Ok(())
    });
    let _ = callback.join();
}

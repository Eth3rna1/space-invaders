//! Keyboard Input Listener Module
//!
//! This module provides a platform-specific abstraction for non-blocking keyboard input,
//! supporting both Windows and non-Windows systems via conditional compilation.
//!
//! # Purpose
//!
//! Designed for integration into a game loop or real-time system, this module enables
//! detection of key presses to control gameplay mechanics such as movement, firing,
//! or game state toggling.
//!
//! # Features
//!
//! - **Cross-platform support**:
//!   - **Windows**: Uses `GetAsyncKeyState` from the WinAPI for instantaneous, low-latency input polling.
//!     This allows for **smooth key stream infiltration**, meaning that holding a key down yields a
//!     consistent and uninterrupted input stream—ideal for fast-paced or continuous input scenarios.
//!   - **Unix-like systems**: Uses `crossterm` to poll and read `KeyEvent` events, filtering for key *presses* only.
//!
//! - Detects common game-relevant keys:
//!   - Arrow keys: `"left"`, `"right"`
//!   - Spacebar: `" "`
//!   - Escape: `"esc"`
//!   - Pause: `"p"`
//!
//! # Usage Example
//!
//! ```rust
//! use crate::listener::get_key;
//!
//! if let Some(key) = get_key() {
//!     match key.as_str() {
//!         "left" => println!("Move left"),
//!         "right" => println!("Move right"),
//!         " " => println!("Jump or shoot"),
//!         "p" => println!("Pause"),
//!         "esc" => println!("Quit"),
//!         _ => {}
//!     }
//! }
//! ```
//!
//! # Notes
//!
//! - On non-Windows systems, only key *presses* are returned. Key *releases* are ignored.
//! - Windows users benefit from smooth and continuous detection of held keys, enhancing
//!   player responsiveness and interaction quality.
//!
//! # Platform Limitations
//!
//! - Windows implementation uses unsafe FFI (`winapi`) to directly access virtual key states.
//! - Non-Windows implementation depends on the event polling behavior of `crossterm`.
use crate::errors::{Error, ErrorKind};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io::Result;
use std::sync::{Arc, RwLock};
use winapi::um::winuser::{
    GetAsyncKeyState, VK_DOWN, VK_ESCAPE, VK_LEFT, VK_RIGHT, VK_SPACE, VK_UP,
};

const VK_P: i32 = 0x50;

#[cfg(target_os = "windows")]
pub fn get_key() -> Option<String> {
    if unsafe { GetAsyncKeyState(VK_SPACE) } & 0x8000u16 as i16 != 0 {
        return Some(" ".to_string());
    }
    // Check if the Left arrow key (VK_LEFT) is pressed
    if unsafe { GetAsyncKeyState(VK_LEFT) } & 0x8000u16 as i16 != 0 {
        return Some("left".to_string());
    }
    // Check if the Right arrow key (VK_RIGHT) is pressed
    if unsafe { GetAsyncKeyState(VK_RIGHT) } & 0x8000u16 as i16 != 0 {
        return Some("right".to_string());
    }
    if unsafe { GetAsyncKeyState(VK_ESCAPE) } & 0x8000u16 as i16 != 0 {
        return Some("esc".to_string());
    }
    if unsafe { GetAsyncKeyState(VK_P) } & 0x8000u16 as i16 != 0 {
        return Some("p".to_string());
    }
    None
}

#[cfg(not(target_os = "windows"))]
pub fn get_key() -> Option<String> {
    if event::poll(std::time::Duration::from_millis(10)).unwrap() {
        if let Ok(Event::Key(KeyEvent { code, kind, .. })) = event::read() {
            if kind == event::KeyEventKind::Release {
                return None;
            }
            return match code {
                KeyCode::Esc => Some("esc".to_string()),
                KeyCode::Right => Some("right".to_string()),
                KeyCode::Left => Some("left".to_string()),
                KeyCode::Char(c) => match c {
                    ' ' => Some(c.to_string()),
                    'p' => Some(c.to_string()),
                    _ => None,
                },
                _ => None,
            };
        }
    }
    None
}

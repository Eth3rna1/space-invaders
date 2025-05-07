use crate::errors::{Error, ErrorKind};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io::Result;
use std::sync::{Arc, RwLock};
use winapi::um::winuser::{GetAsyncKeyState, VK_DOWN, VK_LEFT, VK_RIGHT, VK_SPACE, VK_UP};

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
    // Check if the Up arrow key (VK_UP) is pressed
    //if unsafe { GetAsyncKeyState(VK_UP) } & 0x8000u16 as i16 != 0 {
    //    println!("Up arrow key is pressed!");
    //}
    // Check if the Down arrow key (VK_DOWN) is pressed
    //if unsafe { GetAsyncKeyState(VK_DOWN) } & 0x8000u16 as i16 != 0 {
    //    println!("Down arrow key is pressed!");
    //
    //}
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

//pub fn key_pressed(key: &str) -> bool {
//    if let Some(k) = get_key() {
//        return k == key;
//    }
//    false
//}

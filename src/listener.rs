use crate::errors::{Error, ErrorKind};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io::Result;
use std::sync::{Arc, RwLock};

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
                KeyCode::Char(c) => Some(c.to_string()),
                _ => None,
            };
        }
    }
    None
}

pub fn key_pressed(key: &str) -> bool {
    if let Some(k) = get_key() {
        return k == key;
    }
    false
}

//pub fn keyboard_listener(collector: Arc<RwLock<Vec<Action>>>) -> Result<()> {
//    loop {
//        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
//            if kind == event::KeyEventKind::Release {
//                continue;
//            }
//            {
//                let mut c = collector.write().unwrap();
//                match code {
//                    KeyCode::Esc => break,
//                    KeyCode::Right => c.push(Action::Right),
//                    KeyCode::Left => c.push(Action::Left),
//                    KeyCode::Char(charac) => {
//                        if charac == ' ' {
//                            c.push(Action::Shoot)
//                        }
//                        if charac == 'q' {
//                            c.push(Action::Terminate);
//                        }
//                    }
//                    _ => {}
//                }
//            }
//        }
//    }
//    Ok(())
//}

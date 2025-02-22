use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::sync::{Arc, RwLock};
use std::io::Result;

use crate::ActionQueue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Left,
    Right,
    Shoot,
    Terminate,
}

pub fn keyboard_listener(collector: Arc<RwLock<ActionQueue>>) -> Result<()> {
    loop {
        if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
            if kind == event::KeyEventKind::Release {
                continue;
            }
            {
                let mut c = collector.write().unwrap();
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Right => c.push(Action::Right),
                    KeyCode::Left => c.push(Action::Left),
                    KeyCode::Char(charac) => {
                        if charac == ' ' {
                            c.push(Action::Shoot)
                        }
                        if charac == 'q' {
                            c.push(Action::Terminate);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

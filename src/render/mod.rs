/*
    A double buffer struct that gets frames to store for usage
*/
use std::io::{self, Write, Result};

#[derive(Debug, Clone)]
pub struct Renderer {
    pub frame_count: usize,
    frames: Vec<String>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            frame_count: 0,
        }
    }

    pub fn push(&mut self, frame: String) {
        self.frame_count += 1;
        self.frames.push(frame);
    }

    pub fn swap(&mut self) -> Option<String> {
        if self.frames.is_empty() {
            return None;
        } else if self.frames.len() == 1 {
            return Some(self.frames[0].clone());
        }
        self.frame_count -= 1;
        Some(self.frames.remove(0))
    }
}

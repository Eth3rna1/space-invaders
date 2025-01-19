/*
    A double buffer struct that gets frames to store for usage
*/

#[derive(Debug, Clone)]
pub struct Render {
    pub frame_count : usize,
    frames: Vec<String>
}

impl Render {
    pub fn new() -> Self {
        Self { frames: Vec::new(), frame_count : 0 }
    }

    pub fn push(&mut self, frame: String) {
        self.frame_count += 1;
        self.frames.push(frame);
    }

    pub fn swap(&mut self) -> Option<String> {
        if self.frame_count == 0 {
            return None;
        }
        self.frame_count -= 1;
        Some(self.frames.remove(0))
    }
}

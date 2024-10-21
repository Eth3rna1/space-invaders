/*
    A double buffer struct that gets frames to store for usage
*/

#[derive(Debug, Clone)]
pub struct Render {
    frames : Vec<String>
}

impl Render {
    pub fn new() -> Self {
        Self {
            frames : Vec::new()
        }
    }

    pub fn update(&mut self, frame : String) {
        self.frames.push(frame);
    }

    pub fn output(&mut self) -> Option<String> {
        if self.frames.is_empty() {
            return None;
        }
        Some(self.frames.remove(0))
    }
}

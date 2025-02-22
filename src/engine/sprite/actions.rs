use crate::engine::{sprite::Sprite, Coordinate, Engine};
//use crate::output;
use crate::renderer::Renderer;

use std::sync::{Arc, RwLock};

enum Direction {
    Right,
    Left
}

pub(crate) fn move_side_to_side(sprites : &mut [Sprite], renderer: Arc<RwLock<Renderer>>) {
    let mut direction = Direction::Right;
    loop {
        match direction {
            Direction::Right => {
                if sprites.iter().any(|sprite| sprite.bounding_box.far_right + 1 >= sprite.engine.read().unwrap().width) {
                    direction = Direction::Left;
                    continue;
                }
                for sprite in &mut *sprites {
                    let _ = sprite.move_right();
                }
            },
            Direction::Left  => {
                if sprites.iter().any(|sprite| sprite.bounding_box.far_left == 0) {
                    direction = Direction::Right;
                    continue;
                }
                for sprite in &mut *sprites {
                    let _ = sprite.move_left();
                }
            }
        }
        renderer.write().unwrap().push(sprites[0].engine.read().unwrap().output());
        crate::tool::sleep(0.05);
        //output(&sprites[0].engine.clone(), 0.1);
    }
}

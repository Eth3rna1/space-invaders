use crate::engine::{sprite::Sprite, Coordinate, Engine};
use crate::output;

enum Direction {
    Right,
    Left
}

pub(crate) fn move_side_to_side(sprites : &mut [Sprite]) {
    let mut direction = Direction::Right;
    loop {
        match direction {
            Direction::Right => {
                if sprites.iter().any(|sprite| sprite.bounding_box.far_right + 1 >= sprite.engine.borrow().width) {
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
        output(&sprites[0].engine.clone(), 0.1);
    }
}

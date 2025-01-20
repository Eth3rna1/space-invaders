/*
    Remaking the game Space Invaders in ASCII
*/
mod constants;
mod engine;
mod render;
mod tool;
//mod tests;
mod errors;

use crossterm::terminal; // contains the size() function to measure terminal
use engine::{sprite, Coordinate, Engine};
use std::cell::RefCell;
use std::rc::Rc;

fn output_buffers(renderer: &mut render::Render) {
    tool::clear();
    while let Some(frame) = renderer.swap() {
        print!("{}", frame);
        tool::sleep(0.5);
        tool::refresh();
    }
}

fn main() -> Result<(), errors::Error> {
    let mut renderer = render::Render::new();
    let engine = Engine::new((100, 20)).as_rc();
    let mut sprite1 = sprite::Sprite::new(engine.clone(), vec![(0, 0), (0, 1), (1, 0), (1, 1)])?;
    let sprite2_coordinates: Vec<Coordinate> = {
        let eng = engine.borrow();
        let sprite2_coordinates = vec![
            (eng.width - 1, 0),
            (eng.width - 1, 1),
            (eng.width - 2, 0),
            (eng.width - 2, 1),
        ];
        sprite2_coordinates
    };
    let mut sprite2 = sprite::Sprite::new(engine.clone(), sprite2_coordinates)?;
    sprite1.spawn();
    sprite2.spawn();
    renderer.push(engine.borrow().output());
    sprite1.move_down()?;
    renderer.push(engine.borrow().output());
    sprite1.move_down()?;
    renderer.push(engine.borrow().output());
    sprite2.move_down()?;
    renderer.push(engine.borrow().output());
    sprite1.move_right()?;
    renderer.push(engine.borrow().output());
    sprite2.move_down()?;
    renderer.push(engine.borrow().output());
    sprite2.move_left()?;
    renderer.push(engine.borrow().output());
    output_buffers(&mut renderer);
    Ok(())
}

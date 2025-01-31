/*
    Remaking the game Space Invaders in ASCII

To-Do:
    use the signal_hook crate
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

fn output_buffers(renderer: &mut render::Render, intervals: f64) {
    tool::clear();
    while let Some(frame) = renderer.swap() {
        print!("{}", frame);
        tool::sleep(intervals);
        tool::refresh();
    }
}

fn shooting_alien() -> Result<(), errors::Error> {
    let mut renderer = render::Render::new();
    let engine = Engine::new((100, 20)).as_rc();
    let alien_points: Vec<Coordinate> = {
        // putting the alien on top of the screen
        let eng = engine.borrow();
        let points: Vec<Coordinate> = vec![
            (eng.width / 2 - 1, eng.length / 8),
            (eng.width / 2, eng.length / 8),
            (eng.width / 2 + 1, eng.length / 8),
            (eng.width / 2 - 1, eng.length / 8 + 1),
            (eng.width / 2, eng.length / 8 + 1),
            (eng.width / 2 + 1, eng.length / 8 + 1),
        ];
        points
    };
    let mut alien = sprite::Sprite::new(engine.clone(), alien_points)?;
    alien.spawn();
    let shooter_points: Vec<Coordinate> = {
        let eng = engine.borrow();
        let points: Vec<Coordinate> = vec![
            (eng.width / 2 - 1, eng.length - (eng.length / 8)),
            (eng.width / 2, eng.length - (eng.length / 8)),
            (eng.width / 2 + 1, eng.length - (eng.length / 8)),
        ];
        points
    };
    let mut shooter = sprite::Sprite::new(engine.clone(), shooter_points)?;
    shooter.spawn();
    renderer.push(engine.borrow().output());
    let mut bullet: sprite::Sprite = {
        let eng = engine.borrow();
        let starting_point = vec![(eng.width / 2, eng.length - (eng.length / 8) - 1)];
        drop(eng);
        sprite::Sprite::new(engine.clone(), starting_point)?
    };
    bullet.spawn();
    bullet.move_up()?;
    shooter.spawn();
    renderer.push(engine.borrow().output());
    'main_loop: loop {
        match bullet.move_up() {
            Ok(action) => {
                if action == sprite::State::Collided {
                    bullet.destroy();
                    alien.destroy();
                    renderer.push(engine.borrow().output());
                    break 'main_loop;
                }
            }
            Err(msg) => {
                eprintln!("{}", msg);
                break 'main_loop;
            }
        }
        renderer.push(engine.borrow().output());
    }
    output_buffers(&mut renderer, 0.05);
    Ok(())
}

enum Direction {
    Right,
    Left,
}

fn main() -> Result<(), errors::Error> {
    // simulating the movement of an alien
    let mut engine = Engine::new((100, 20)).as_rc();
    let mut renderer = render::Render::new();
    let mut direction = Direction::Right;
    let mut alien = sprite::Sprite::new(engine.clone(), vec![(1, 1), (1, 2), (2, 1), (2, 2)])?;
    alien.spawn();
    renderer.push(engine.borrow().output());
    loop {
        match direction {
            Direction::Right => {
                if alien.move_right().is_err() {
                    if alien.move_down().is_err() {
                        break;
                    };
                    direction = Direction::Left;
                }
            }
            Direction::Left => {
                if alien.move_left().is_err() {
                    if alien.move_down().is_err() {
                        break;
                    };
                    direction = Direction::Right;
                }
            }
        }
        renderer.push(engine.borrow().output());
    }
    output_buffers(&mut renderer, 0.01);
    Ok(())
}

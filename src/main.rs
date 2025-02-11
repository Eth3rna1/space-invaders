/*
    Remaking the game Space Invaders in ASCII

To-Do:
    use the signal_hook crate
*/
mod constants;
mod engine;
mod render;
mod listener;
mod tool;
//mod tests;
mod errors;

use crossterm::terminal; // contains the size() function to measure terminal
use errors::Error;
use render::Renderer;
use engine::{
    sprite::{self, Sprite},
    Coordinate,
    Engine
};
use listener::Action;
use std::thread;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use std::rc::Rc;

pub fn output(engine: &Rc<RefCell<Engine>>, wait: f64) {
    //tool::clear();
    println!("{}", engine.borrow().output());
    tool::sleep(wait);
    tool::refresh();
}

fn output_buffers(renderer: &mut Renderer, intervals: f64) {
    while let Some(frame) = renderer.swap() {
        print!("{}", frame);
        tool::sleep(intervals);
        tool::refresh();
    }
}

fn spawn_alien(engine: Rc<RefCell<Engine>>, position: Vec<Coordinate>) -> Result<Sprite, Error> {
    let mut sprite = Sprite::new(engine, position)?;
    sprite.spawn();
    Ok(sprite)
}

fn spawn_aliens(engine: Rc<RefCell<Engine>>, count: usize) -> Result<Vec<Sprite>, Error> {
    let width = { engine.borrow().width };
    let difference = width / count;
    let mut aliens = Vec::new();
    for row in 2..5 {
        let mut current_x = 0;
        while current_x + difference < width {
            aliens.push(spawn_alien(engine.clone(), vec![(current_x, row)])?);
            current_x += difference;
        }
    }
    Ok(aliens)
}

fn spawn_shooter(engine: Rc<RefCell<Engine>>) -> Result<Sprite, Error> {
    let position = {
        let eng = engine.borrow();
        vec![
            (eng.width / 2, eng.length - (eng.length / 7)),
            (eng.width / 2 + 1, eng.length - (eng.length / 7)),
            (eng.width / 2 - 1, eng.length - (eng.length / 7)),
            (eng.width / 2, eng.length - (eng.length / 7 + 1))
        ]
    };
    let mut shooter = Sprite::new(engine, position)?;
    shooter.spawn();
    Ok(shooter)
}

fn main() -> Result<(), Error> {
    tool::clear();
    let engine = Engine::new((100, 20)).as_rc();
    let renderer = Renderer::new();
    let actions: Arc<RwLock<Vec<listener::Action>>> = Arc::new(RwLock::new(Vec::new()));
    let a_clone = actions.clone();
    let _ = thread::spawn(move || {
        listener::keyboard_listener(a_clone);
    });
    let mut shooter = spawn_shooter(engine.clone())?;
    let mut aliens = spawn_aliens(engine.clone(), 10)?;
    output(&engine, 0.01);
    //sprite::actions::move_side_to_side(&mut aliens);
    loop {
        let mut acts = actions.write().unwrap();
        if acts.is_empty() {
            continue;
        }
        match acts.remove(0) {
            Action::Right => { shooter.move_right(); },
            Action::Left => { shooter.move_left(); },
            Action::Shoot => println!("PLAYER CHOSE TO SHOOT")
        }
        output(&engine, 0.01);
    }
    Ok(())
}

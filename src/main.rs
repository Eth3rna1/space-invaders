/*
    Remaking the game Space Invaders in ASCII

To-Do:
    use the signal_hook crate
*/
mod constants;
mod engine;
mod renderer;
mod listener;
mod tool;
//mod tests;
mod errors;

use errors::{Error, ErrorKind};
use crossterm::terminal;
//use errors::Error;
use renderer::Renderer;
use engine::{
    sprite::{self, Sprite},
    Coordinate,
    Engine
};
use listener::Action;
use std::thread::{self, JoinHandle};
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use std::process::exit;
use std::rc::Rc;


const DELTA_TIME: f64 = 0.5;
const PLANE_DIMENSIONS: Coordinate = (100, 20); // Coordinate is (usize, usize)
const ALIEN_COUNT: usize = 7;

//pub fn output(engine: &Rc<RefCell<Engine>>, wait: f64) {
//    //tool::clear();
//    println!("{}", engine.borrow().output());
//    tool::sleep(wait);
//    tool::refresh();
//}

//fn output_buffers(renderer: &mut Renderer, intervals: f64) {
//    while let Some(frame) = renderer.swap() {
//        print!("{}", frame);
//        tool::sleep(intervals);
//        tool::refresh();
//    }
//}

fn spawn_alien(engine: Arc<RwLock<Engine>>, position: Vec<Coordinate>) -> Result<Sprite, Error> {
    let mut sprite = Sprite::new(engine, position)?;
    sprite.spawn();
    Ok(sprite)
}

fn spawn_aliens(engine: Arc<RwLock<Engine>>, count: usize) -> Result<Vec<Sprite>, Error> {
    let width = { engine.read().unwrap().width };
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

fn spawn_shooter(engine: Arc<RwLock<Engine>>) -> Result<Sprite, Error> {
    let position = {
        let eng = engine.read().unwrap();
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

#[derive(Debug, Clone)]
pub struct ActionQueue {
    __actions : Vec<Action>,
    __capacity: usize,
    __memoized_len: usize
}

impl ActionQueue {
    pub fn new(__capacity: usize) -> Self {
        Self {
            __actions: Vec::new(),
            __capacity,
            __memoized_len: 0
        }
    }

    pub fn push(&mut self, action: Action) {
        if self.__memoized_len >= self.__capacity {
            let _ = self.__actions.remove(0);
            self.__memoized_len -= 1;
        }
        self.__actions.push(action);
        self.__memoized_len += 1;
    }

    pub fn remove(&mut self, index: usize) -> Action {
        self.__memoized_len -= 1;
        self.__actions.remove(index)
    }

    pub fn is_empty(&self) -> bool {
        self.__memoized_len == 0
    }
}

fn main() -> Result<(), Error> {
    tool::clear();
    terminal::enable_raw_mode().expect("Could not enable terminal raw mode");
    let engine: Arc<RwLock<Engine>> = Engine::new(PLANE_DIMENSIONS).as_arc();
    let acts: Arc<RwLock<ActionQueue>> = Arc::new(RwLock::new(ActionQueue::new(ACTION_QUEUE_CAPACITY)));
    let renderer: Arc<RwLock<Renderer>> = Renderer::new().as_arc();
    let mut shooter: Sprite = spawn_shooter(engine.clone()).expect("Could not spawn shooter");

    let r_clone: Arc<_> = renderer.clone();
    //let e_clone = engine.clone();
    let _ = thread::spawn(move || {
        loop {
            let mut r = r_clone.write().unwrap();
            if r.is_empty() {
                continue;
            }
            if let Some(frame) = r.swap() {
                print!("{}", frame);
                tool::sleep(DELTA_TIME);
                tool::refresh();
            }
            //print!("{}", r.swap());
            //tool::sleep(DELTA_TIME);
            //tool::refresh();
        }
    });

    let a_clone: Arc<_> = acts.clone();
    //listener daemon thread
    let _ = thread::spawn(move || {
        listener::keyboard_listener(a_clone);
    });
    let e_clone: Arc<_> = engine.clone();
    let r_clone: Arc<_> = renderer.clone();
    let _ = thread::spawn(move || {
        let mut aliens: Vec<Sprite> = spawn_aliens(e_clone, ALIEN_COUNT).expect("Could not spawn aliens");
        sprite::actions::move_side_to_side(&mut aliens, r_clone);
    });
    loop {
        tool::sleep(DELTA_TIME);
        let mut actions = acts.write().unwrap();
        if actions.is_empty() {
            continue;
        }
        match actions.remove(0) {
            Action::Terminate => break,
            Action::Left => { shooter.move_left(); },
            Action::Right => { shooter.move_right(); },
            Action::Shoot => {
                let far_right = shooter.bounding_box.far_right;
                let far_left = shooter.bounding_box.far_left;
                let far_top = shooter.bounding_box.far_top;
                //let far_bottom = shooter.bounding_box.far_bottom;
                let front_of_gun: Coordinate = (far_right - ((far_right - far_left) / 2), far_top - 2);
                //engine.write().unwrap().spawn(front_of_gun);
                let engine_pointer = engine.clone();
                let _ = thread::spawn(move || {
                    let mut bullet = Sprite::new(engine_pointer, vec![front_of_gun]).expect("Could not spawn bullet");
                    bullet.spawn();
                    loop {
                        if let Err(error) = bullet.move_up() {
                            if error.kind() == ErrorKind::OutOfBounds {
                                bullet.destroy();
                            }
                            break;
                        }
                        tool::sleep(0.05);
                    }
                });
            }
        }
    }
    terminal::disable_raw_mode().expect("Could not disable terminal raw mode");
    Ok(())
}

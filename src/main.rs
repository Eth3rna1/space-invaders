/*
    Previous problem, needed to store frames
*/
mod render;
mod engine;
mod tool;
use std::thread::{self, JoinHandle};
use std::sync::{Arc, RwLock};
use render::Render;


fn main() {
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let render = Arc::new(RwLock::new(render::Render::new()));
    for i in 0..2 {
        let render_ptr = Arc::clone(&render);
        let j = thread::spawn(move || {
            let mut r = render_ptr.write().unwrap();
            r.update(String::from("+"));
        });
        threads.push(j);
    }
    for t in threads {
        let _ = t.join();
    }
    loop {
        let mut prender = render.write().unwrap();
        let value = prender.output();
        if value.is_none() {
            break;
        }
        println!("{}", value.unwrap());
        tool::clear();
    }
}

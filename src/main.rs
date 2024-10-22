/*
    Remaking the game Space Invaders in ASCII
*/
mod constants;
mod engine;
mod render;
mod tool;
use render::Render;

fn main() {
    // testing out the pool functions
    let dimensions: (usize, usize) = (20, 10);
    let mut eng = engine::Engine::new(dimensions);
    let mut render = Render::new();
    let mut pool: [(usize, usize); 3] = [(1, 0), (1, 1), (1, 2)];
    engine::spawn_pool(&mut eng, &pool);
    render.update(eng.output());
    engine::move_pool_right(&mut eng, &mut pool);
    render.update(eng.output());
    loop {
        if let Some(frame) = render.output() {
            println!("{}", frame);
            //tool::clear();
            tool::sleep(0.05);
        } else {
            break;
        }
    }
}

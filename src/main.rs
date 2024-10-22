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
    let dimensions: (usize, usize) = (30, 10);
    let mut eng = engine::Engine::new(dimensions);
    let mut render = Render::new();
    let mut pool: [(usize, usize); 3] = [(0, 0), (0, 1), (0, 2)];
    engine::spawn_pool(&mut eng, &pool);
    for i in 0..eng.width - 1 {
        render.update(eng.output());
        engine::move_pool_right(&mut eng, &mut pool);
    }
    render.update(eng.output());
    tool::clear();
    while let Some(frame) = render.output() {
        println!("{}", frame);
        tool::refresh();
        tool::sleep(0.05);
    }
}

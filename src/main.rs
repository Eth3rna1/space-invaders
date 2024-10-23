/*
    Remaking the game Space Invaders in ASCII
*/
mod constants;
mod engine;
mod render;
mod tool;
use crossterm::terminal; // contains the size() function to measure terminal
use engine::{pool, Engine};
use render::Render;

enum Direction {
    Down,
    Up
}

fn zigzag(engine : &mut Engine, render : &mut Render, pool : &mut [(usize, usize)]) {
    let height = 5;
    let mut tmp = 0;
    let mut direction = Direction::Down;
    for _ in 0..engine.width - 1 {
        match tmp {
            height => {
                tmp = 0;
                match direction {
                    Direction::Down => direction = Direction::Down,
                    Direction::Up   => direction = Direction::Up
                }
            },
            0 | _  => {
                match direction {
                    Direction::Down => pool::move_pool_down(engine, pool),
                    Direction::Up   => pool::move_pool_up(engine, pool)
                }
            }
        }
        tmp += 1;
        pool::move_pool_right(engine, pool);
        render.update(engine.output());
    }
}

fn single_zigzag() {
    let mut render = Render::new();
    let dimensions: (usize, usize) = (30, 10);
    let mut engine = Engine::new(dimensions);
    // creating a pool to not have to do mutations manually
    let mut pixel: [(usize, usize); 1] = [(0, engine.length / 2)];
    let height = 2; // height difference of 5 pixels
    let mut tmp: i32 = 0; // messures the height
    engine.spawn(constants::PIXEL_CHAR, pixel[0]);
    render.update(engine.output());
    let mut direction = Direction::Down;
    for w in 0..engine.width - 1 {
        if tmp.abs() == height {
            tmp = 0;
            match direction {
                Direction::Down => direction = Direction::Up,
                Direction::Up   => direction = Direction::Down
            }
        }
        pool::move_pool_right(&mut engine, &mut pixel);
        match direction {
            Direction::Down => {
                pool::move_pool_down(&mut engine, &mut pixel);
                tmp += 1;
            },
            Direction::Up   => {
                pool::move_pool_up(&mut engine, &mut pixel);
                tmp -= 1;
            }
        }
        engine.spawn('-', (w, engine.length / 2));
        render.update(engine.output());
    }
    tool::clear();
    while let Some(frame) = render.output() {
        tool::refresh();
        println!("{}", frame);
        tool::sleep(0.05);
    }
}

fn pool() {
    // testing out the pool functions
    let dimensions: (usize, usize) = (30, 10);
    let mut engine = Engine::new(dimensions);
    let mut render = Render::new();
    let mut pool: [(usize, usize); 3] = [(0, 0), (0, 1), (0, 2)];
    pool::spawn_pool(&mut engine, &pool);
    zigzag(&mut engine, &mut render, &mut pool);
    tool::clear();
    if render.frame_count == 0 {
        eprintln!("No frames were produced");
        std::process::exit(1);
    }
    while let Some(frame) = render.output() {
        println!("{}", frame);
        tool::refresh();
        tool::sleep(0.05);
    }
}


fn main() {
    single_zigzag();
}

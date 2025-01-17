use crate::engine::Engine;
use crate::math; // fraction()
use crate::engine::sprite;
use crate::render::Render;
use crate::tool; // clear() refresh()
use crate::constants;


enum Direction {
    Down,
    Up,
    Right,
    Left
}


/// Zig zagging a single pixel, then line, then group of pixels
pub(crate) fn zigzag() {
    let dimensions: (usize, usize) = (100, 20);
    let mut engine = Engine::new(dimensions);

    let mut sprite: [(usize, usize); 3] = [
        (0, engine.length / 2 - 1),
        (0, engine.length / 2),
        (0, engine.length / 2 + 1)
    ];

    sprite::spawn_sprite(&mut engine, &mut sprite);
    let mut render = Render::new();
    //let mut point = (0, engine.length / 2); // starts at right middle
    //engine.spawn(point);
    render.update(engine.output());
    let height = 5;
    let mut count: i32 = 0;
    let mut direction = Direction::Down;
    for _ in 0..engine.width - 1 {
        if count.abs() == height {
            // inverting the direction
            match direction {
                Direction::Up   => direction = Direction::Down,
                Direction::Down => direction = Direction::Up,
                _ => ()
            }
        }
        match direction {
            Direction::Up   => sprite::move_sprite_up(&mut engine, &mut sprite),
            Direction::Down => sprite::move_sprite_down(&mut engine, &mut sprite),
            _ => ()
        }
        sprite::move_sprite_right(&mut engine, &mut sprite);
        render.update(engine.output());
        match direction {
            Direction::Up => count -= 1,
            Direction::Down => count += 1,
            _ => ()
        }
    }
    tool::clear();
    while let Some(frame) = render.output() {
        tool::refresh();
        print!("{}", frame);
        tool::sleep(0.03);
    }
}

/// Problem: When obtaining the slope, I have to have whole numbers
///
/// Problem Situation:
///     One pixel apart distance in height
///
///    (start) ########
///                    #
///                     ########## (end)
pub(crate) fn line() {
    let mut dimension = (100usize, 20usize);
    let mut engine = Engine::new(dimension);
    let mut render = Render::new();
    todo!();
    tool::clear();
    while let Some(frame) = render.output() {
        tool::refresh();
        println!("{}", frame);
        tool::sleep(0.05);
    }
}


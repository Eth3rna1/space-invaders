use crate::engine::sprite::State;
use crate::engine::Coordinate;
use crate::engine::Engine;
use crate::entities::{
    Bullet, Shooter, Speedster,
    {
        farthest_left_alien, farthest_right_alien, find_alien_and_destroy, spawn_aliens, Alien,
        Direction,
    },
};
use crate::errors::{Error, ErrorKind};
use crate::listener::get_key;
use crate::utils;
use crate::{
    ALIEN_COL_COUNT, ALIEN_STEP_PER_DELTA, BACKGROUND_CHAR, BULLET_STEP_PER_DELTA, PIXEL_CHAR,
    SHOOTER_STEP_PER_DELTA, SPEEDSTER_STEP_PER_DELTA,
};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SpaceInvaders {
    pub(crate) aliens: Vec<Alien>,
    pub(crate) alien_xd: f32,
    pub(crate) alien_direction: Direction,
    pub(crate) game_over: bool,
    pub(crate) shooter: Shooter,
    pub(crate) key: Option<String>,
    pub(crate) bullets: Vec<Bullet>,
    pub(crate) speedster: Speedster,
    pub(crate) engine: Rc<RefCell<Engine>>,
    pub(crate) width: usize,
    pub(crate) game_won: bool,
    pub(crate) game_quit: bool,
    pub(crate) game_paused: bool,
    pub(crate) game_initialized: bool,
}

impl SpaceInvaders {
    pub fn new(dimensions: (usize, usize)) -> Result<Self, Error> {
        let engine = Engine::new(dimensions).as_rc();
        let width = { engine.borrow().width };
        let shooter: Shooter = {
            let position: Vec<Coordinate> = {
                let eng = engine.borrow();
                vec![
                    (eng.width / 2, eng.height - (eng.height / 7)),
                    (eng.width / 2 + 1, eng.height - (eng.height / 7)),
                    (eng.width / 2 - 1, eng.height - (eng.height / 7)),
                    (eng.width / 2, eng.height - (eng.height / 7) - 1),
                ]
            };
            Shooter::new(engine.clone(), position, SHOOTER_STEP_PER_DELTA)?
        };
        let speedster: Speedster = Speedster::new(engine.clone(), SPEEDSTER_STEP_PER_DELTA)?;
        Ok(Self {
            aliens: spawn_aliens(engine.clone(), ALIEN_COL_COUNT, ALIEN_STEP_PER_DELTA)?,
            //aliens: Vec::new(),
            alien_xd: 0.0,
            engine,
            width,
            game_quit: false,
            shooter,
            alien_direction: Direction::Right,
            key: None,
            speedster,
            game_won: false,
            game_over: false,
            game_paused: true,
            game_initialized: false,
            bullets: Vec::new(),
        })
    }

    pub fn set_up(&mut self) {
        utils::clear();
        for mut alien in &mut self.aliens {
            let _ = alien.spawn();
        }
        self.shooter.spawn();
    }

    /// Memoizes the key pressed
    pub fn handle_input(&mut self) {
        self.key = if let Some(key) = get_key() {
            self.game_initialized = true;
            self.game_paused = if key == "p" { true } else { false };
            if key == "esc" {
                self.game_quit = true;
            }
            Some(key)
        } else {
            None
        }
    }

    /// checking key pressed and spawning bullets if a space is pressed
    fn _update_upon_key_press(&mut self, delta_time: f32) {
        if let Some(key) = &self.key {
            match key.as_str() {
                " " => {
                    // spawning a bullet
                    if let Ok(mut bullet) = Bullet::new(
                        self.engine.clone(),
                        self.shooter.head(),
                        BULLET_STEP_PER_DELTA,
                    ) {
                        if let Ok(_) = bullet.spawn() {
                            self.bullets.push(bullet);
                        }
                    }
                }
                "left" | "right" => {
                    let _ = self.shooter.step(&key, delta_time);
                    if let Some(coordinate) = self.shooter.step(&key, delta_time) {
                        self.game_over = true;
                    }
                }
                _ => {}
            }
        }
    }

    /// moving aliens
    pub fn _move_aliens(&mut self, delta_time: f32) {
        if self.aliens.is_empty() {
            return;
        }
        let offset: f32 = ALIEN_STEP_PER_DELTA * delta_time;
        let step = (self.alien_xd + offset) as usize - self.alien_xd as usize;
        if step == 0 {
            self.alien_xd += offset;
            return;
        }
        self.alien_xd = 0.0;
        match self.alien_direction {
            Direction::Left => {
                if farthest_left_alien(&self.aliens).far_left() == 0 {
                    self.alien_direction = Direction::Right;
                    return;
                }
                for a in (0..self.aliens.len()).rev() {
                    // making the step negative to move left
                    if let Some(coordinate) = self.aliens[a].step(0 - step as i32) {
                        self.aliens[a].destroy();
                        let _ = self.aliens.remove(a);
                        // checking for bullet collisions
                        for i in (0..self.bullets.len()).rev() {
                            if self.bullets[i].contains(coordinate) {
                                self.bullets[i].destroy();
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                if farthest_right_alien(&self.aliens).far_right() == self.width - 1 {
                    self.alien_direction = Direction::Left;
                    // moving the aliens down for more pressure
                    for a in 0..self.aliens.len() {
                        match self.aliens[a].move_y(1) {
                            Ok(state) => match state {
                                State::Collided(coordinate) => {
                                    if self.shooter.contains(coordinate) {
                                        self.game_over = true;
                                        return;
                                    }
                                }
                                _ => (),
                            },
                            Err(error) => match error.kind() {
                                ErrorKind::OutOfBounds => todo!(),
                                _ => (),
                            },
                        }
                    }
                    return;
                }
                for a in (0..self.aliens.len()).rev() {
                    // making the step negative to move left
                    if let Some(coordinate) = self.aliens[a].step(step as i32) {
                        self.aliens[a].destroy();
                        let _ = self.aliens.remove(a);
                        // checking for bullet collisions
                        for i in (0..self.bullets.len()).rev() {
                            if self.bullets[i].contains(coordinate) {
                                self.bullets[i].destroy();
                                let _ = self.bullets.remove(i);
                            }
                        }
                    }
                }
            }
        }
        // getting the aliens to shoot at the player
        let alien_in_same_x: &Alien = {
            let shooter_xs = self.shooter.xs();
            let mut aliens_with_same_x: Vec<&Alien> = Vec::new();
            for alien in self.aliens.iter() {
                //if shooter_x == alien.x() {
                if shooter_xs.contains(&alien.x()) {
                    aliens_with_same_x.push(alien);
                }
            }
            if aliens_with_same_x.is_empty() {
                // exits the function
                return;
            }
            let mut farthest_down: &Alien = &aliens_with_same_x[0];
            for alien in aliens_with_same_x.iter() {
                if alien.far_bottom() > farthest_down.far_bottom() {
                    farthest_down = alien
                }
            }
            farthest_down
        };
        if let Ok(mut b) = Bullet::new(
            self.engine.clone(),
            alien_in_same_x.head(),
            BULLET_STEP_PER_DELTA,
        ) {
            let _ = b.spawn();
            self.bullets.push(b.to_alien_bullet());
        }
    }

    /// moving all bullets
    pub fn _move_bullets(&mut self, delta_time: f32) {
        for i in 0..self.bullets.len() {
            if self.bullets[i].is_destroyed() {
                continue;
            }
            if let Some(coordinate) = self.bullets[i].step(delta_time) {
                // coordinate of the sprite it collided with
                //if self.aliens.find_and_destroy(coordinate) {
                if find_alien_and_destroy(&mut self.aliens, coordinate) {
                    self.bullets[i].destroy();
                } else if self.shooter.contains(coordinate) {
                    // an alien bullet that hit the player
                    self.game_over = true;
                    return;
                } else if self.speedster.contains(coordinate) {
                    // speedster vs the player in the end game
                    //self.game_over = true;
                    //self.speedster.destroy();
                    if !self.bullets[i].is_alien_bullet() {
                        self.speedster.was_hit();
                    }
                    if self.speedster.is_dead() {
                        self.game_won = true;
                    }
                    self.bullets[i].destroy();
                } else {
                    // collided with another bullet
                    for a in 0..self.bullets.len() {
                        if self.bullets[a].is_destroyed() {
                            continue;
                        }
                        let mut bullet = &mut self.bullets[a];
                        if bullet.contains(coordinate) && bullet.is_alien_bullet() {
                            bullet.destroy();
                            self.bullets[i].destroy();
                        }
                    }
                }
            }
        }
        for i in (0..self.bullets.len()).rev() {
            if self.bullets[i].is_destroyed() {
                let _ = self.bullets.remove(i);
            }
        }
    }

    fn __if_sprite_contained_coordinate_destroyed(&mut self, coordinate: Coordinate) -> bool {
        if find_alien_and_destroy(&mut self.aliens, coordinate) {
            // function already does the destroying
            return true;
        } else if self.shooter.contains(coordinate) {
            // an alien bullet that hit the player
            self.game_over = true;
            return true;
        } else if self.speedster.contains(coordinate) {
            // speedster vs the player in the end game
            //self.game_over = true;
            //self.speedster.destroy();
            self.speedster.was_hit();
            self.game_won = true;
            return true;
        } else {
            // collided with bullet
            for a in 0..self.bullets.len() {
                if self.bullets[a].is_destroyed() {
                    continue;
                }
                if self.bullets[a].contains(coordinate) {
                    self.bullets[a].destroy();
                    return true;
                }
            }
        }
        false
    }

    /// Spawns the speedster once all aliens have been killed
    pub fn _spawn_speedster_if_end_game(&mut self, delta_time: f32) {
        if !self.aliens.is_empty() {
            // not yet an end game either because
            // aliens are still alive or there are
            // bullets on the plane
            return;
        }
        let stages = self.speedster.stages();
        let stages_completed = self.speedster.stages_completed();
        if [0, 1].contains(&stages_completed) {
            println!(
                "{:^100}",
                format!(
                    "SPEEDSTER LIVES: {} / {}",
                    stages - stages_completed,
                    stages
                )
            );
        } else {
            // in stage 3
            match self.speedster.stage_3_phase() {
                0 => (),
                1 => println!("{:^100}", "Hmmmm? It seems like its snowing..."),
                2 => println!("{:^100}", "Watch out for the falling blocks!!!"),
                3 => println!("{:^100}", "Quick! Shoot him!"),
                _ => panic!("Invalid phase was returned"),
            }
        }
        //if !self.bullets.is_empty() && !self.speedster.is_initialized() {
        if self
            .bullets
            .iter()
            .any(|b| !b.is_destroyed() && !b.is_alien_bullet())
            && !self.speedster.is_spawned()
        {
            // if any bullet is not destroyed and it happens to be the players bullet
            // then the sprite will not be spawned
            return;
        }
        self.speedster.spawn();
        if let Some(coordinate) = self.speedster.step(delta_time, &mut self.bullets) {
            if self.__if_sprite_contained_coordinate_destroyed(coordinate) {
                return;
            }
            //if self.shooter.contains(coordinate) {
            //    self.game_over = true;
            //    return;
            //}
            //self.speedster.destroy();
            //self.speedster.reset_position();
            //self.speedster.next_stage();
            if self.speedster.is_dead() {
                self.game_won = true;
                //return;
            }
            return;
        }
        //if self.speedster.x() == self.shooter.x() {
        //if self.shooter.xs().iter().any(|x| *x == self.speedster.x()) {
        //    // sprites are in the same x position, means the sprite should shoot
        //    if let Ok(mut b) = Bullet::new(
        //        self.engine.clone(),
        //        self.speedster.head(),
        //        BULLET_STEP_PER_DELTA,
        //    ) {
        //        let mut bullet = b.to_alien_bullet();
        //        let _ = bullet.spawn();
        //        self.bullets.push(bullet);
        //    }
        //}
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.game_paused {
            return;
        }
        {
            // sends the key to the shooter or spawns new bullets
            self._update_upon_key_press(delta_time);
        }
        {
            //// moves aliens, taking into account collisions with bullets,
            //// if collides with a bullet, both the alien and bullet get destroyed
            self._move_aliens(delta_time);
            //self.aliens = Vec::new();
        }
        {
            // If all aliens are dead, then this sprite will spawn
            // and an end game will commence
            self._spawn_speedster_if_end_game(delta_time);
        }
        {
            // moves bullets, taking into account collisions with other sprites.
            // If collides with other sprites, both the bullet and sprite gets
            // destroyed.
            // If the bullet collides with the speedster, the game is automatically
            // a win for the player.
            self._move_bullets(delta_time);
        }
    }

    pub fn draw(&mut self) {
        if !self.game_initialized {
            println!("Welcome to Space Invaders! Press any game key to start!");
        } else if self.game_paused {
            println!("Game is paused. Press any game key to continue...");
        }
        print!(
            "{}",
            self.engine.borrow().display(PIXEL_CHAR, BACKGROUND_CHAR)
        );
        utils::refresh();
    }

    pub fn game_over(&mut self) -> bool {
        (self.aliens.is_empty() && self.speedster.is_dead()) || self.game_over || self.game_quit
    }

    pub fn won(&self) -> bool {
        self.game_won
    }
}

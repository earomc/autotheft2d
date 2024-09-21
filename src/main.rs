use autotheft2d::{Character, Direction};
use macroquad::prelude::*;

const WINDOW_HEIGHT: i32 = 720;
const WINDOW_WIDTH: i32 = 1280;

fn window_conf() -> Conf {
    Conf {
        window_title: "AutoTheft2D".into(),
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let character_sprite = load_texture("assets/character.png").await.unwrap();
    let mut character = Character::new(character_sprite);
    let mut direction_iter = Direction::SOUTH.into_iter();
    let mut timer_current = 0.;
    loop {
        timer_current += get_frame_time();
        if timer_current >= 0.5 {
            character.facing = direction_iter.next().unwrap();
            println!("Advanced to {:?} ", character.facing);
            timer_current -= 0.5
        }
        character.draw();
        next_frame().await
    }
}



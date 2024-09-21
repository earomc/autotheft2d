use autotheft2d::{map::{Map, Tile}, Character, Direction};
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
    let tile_map = load_texture("assets/map_tiles.png").await.unwrap();
    let mut character = Character::new(character_sprite);
    
    let map = Map::new(&tile_map);
   
    let mut direction_iter = Direction::SOUTH.into_iter();
    let mut timer_current = 0.;
    character.pos = (-256., 0.);
    loop {
        timer_current += get_frame_time();
        if timer_current >= 0.1 {
            character.facing = direction_iter.next().unwrap();
            println!("Advanced to {:?} ", character.facing);
            println!("Character position: {:?}", character.pos);
            character.pos.0 += 10.;
            
            
            timer_current -= 0.1
        }
        map.draw(character.pos);
        //tile.draw_at_screenspace(256.,0.);
        character.draw();
        
        next_frame().await
    }
}



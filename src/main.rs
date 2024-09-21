use autotheft2d::{controller::Controller, map::Map, player::Player, vehicle::Vehicle};
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
    let player_sprite = load_texture("assets/player.png").await.unwrap();
    let tile_map = load_texture("assets/map_tiles.png").await.unwrap();
    let vehicle_sprite = load_texture("assets/Car-0001.png").await.unwrap();
    let mut character = Player::new(player_sprite);
    
    let map = Map::new(&tile_map);
    let vehicle = Vehicle::new(&vehicle_sprite);
    let mut controller = Controller::default();
    loop {
        controller.handle_key_inputs(&mut character);
        map.draw(character.pos);
        character.draw();
        vehicle.draw(character.pos);
        next_frame().await
    }
}

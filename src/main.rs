use std::{cell::RefCell, rc::Rc};

use autotheft2d::{controller::ControllerState, draw::Draw, map::Map, player::Player, vehicle::Vehicle, Update};
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
    let mut player = Player::new(&player_sprite);
    
    let map = Map::new(&tile_map);
    let mut vehicles = Vec::new();
    let vehicle = Rc::new(RefCell::new(Vehicle::new(&vehicle_sprite)));
    vehicles.push(vehicle);
    let mut controller = ControllerState::default();
    loop {
        controller.handle_key_inputs(&mut player);
        clear_background(DARKGREEN);
        map.draw(player.pos);
        
        if is_key_pressed(KeyCode::F) {
            if let Some(vehicle) = player.in_vehicle.clone() {
                player.leave_vehicle(vehicle);
                println!("left vehicle")
            } else {
                if let Some(vehicle) = vehicles.iter().find(|v| v.borrow().pos.distance(player.pos) < 100.) {
                    player.enter_vehicle(vehicle.clone()); // increase ref-count.
                    println!("entered vehicle");
                }
            }
        }
        player.draw();
        player.update();
        vehicles.iter().for_each(|v| {
            let mut v = v.borrow_mut();
            v.draw_at_world_space(player.pos);
            v.update();
        });
        next_frame().await
    }
}

use autotheft2d::{
    controller::ControllerDirectionState, draw::Draw, map::Map, player::Player,
    util::SpiralIterator, vehicle::Vehicle, weapons::Weapon, Update,
};
use macroquad::prelude::*;
use std::{cell::RefCell, rc::Rc};
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

macro_rules! include_texture {
    ($path:expr) => {
        Texture2D::from_file_with_format(include_bytes!($path), None)
    };
}

#[macroquad::main(window_conf)]
async fn main() {
    let player_sprite = include_texture!("../assets/player.png");
    let tile_map = include_texture!("../assets/map_tiles.png");
    let vehicle_sprite = include_texture!("../assets/Car-0001.png");
    let mut player = Player::new(&player_sprite);

    let map = Map::new(&tile_map, 1024);
    let mut vehicles = Vec::new();
    let vehicle = Rc::new(RefCell::new(Vehicle::new(&vehicle_sprite)));
    vehicles.push(vehicle);
    let mut controller = ControllerDirectionState::default();

    let weapon_sprite = include_texture!("../assets/pistol.png");
    let mut weapon = Weapon::new(
        &weapon_sprite,
        2.,
    );
    loop {
        controller.handle_key_inputs(&mut player);
        clear_background(DARKGREEN);
        map.draw(player.pos, 5);

        if is_key_pressed(KeyCode::F) {
            if let Some(vehicle) = player.in_vehicle.clone() {
                player.leave_vehicle(vehicle);
                println!("left vehicle")
            } else {
                if let Some(vehicle) = vehicles
                    .iter()
                    .find(|v| v.borrow().pos.distance(player.pos) < 100.)
                {
                    player.enter_vehicle(vehicle.clone()); // increase ref-count.
                    println!("entered vehicle");
                }
            }
        }
        player.draw();
        weapon.draw_at_world_space(player.pos);
        weapon.pos = player.pos;
        weapon.pos.y += 30.;
        player.update();
        vehicles.iter().for_each(|v| {
            let mut v = v.borrow_mut();
            v.draw_at_world_space(player.pos);
            v.update();
        });
        if let Some(vehicle) = player.in_vehicle.clone() {
            let vehicle = vehicle.borrow();
            let velocity = vehicle.velocity;
            let speed_text = &format!("{:.2} km/h", velocity * 3.6);
            draw_text(
                speed_text,
                screen_width() / 2.,
                screen_height() - 140.,
                50.,
                WHITE,
            );
            draw_text(
                &format!("Acceleration: {:.10}", vehicle.acceleration),
                screen_width() / 2.,
                screen_height(),
                50.,
                WHITE,
            );
        }
        draw_text(
            &format!("pos: {:.2?}", player.pos),
            screen_width() / 2.,
            screen_height() - 40.,
            50.,
            WHITE,
        );
        draw_text(
            &format!("pos (tile): {:.2?}", map.to_tile_index_pos(player.pos)),
            screen_width() / 2.,
            screen_height() - 70.,
            50.,
            WHITE,
        );

        next_frame().await
    }
}

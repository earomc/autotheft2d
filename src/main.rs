use autotheft2d::{
    collide::{LineSegment, Object2D}, controller::ControllerDirectionState, draw::{draw_vector, Draw}, map::Map, player::Player, util::{mouse_direction, SpiralIterator}, vehicle::Vehicle, weapons::{Projectile, Weapon}, Update, World
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

// Example usage
fn test_collision() {
    // Example with a custom Object2D
    let custom_object = Object2D {
        position: Vec2::new(100.0, 100.0),
        shape: vec![
            LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 0.0)),
            LineSegment::new(Vec2::new(50.0, 0.0), Vec2::new(50.0, 50.0)),
            LineSegment::new(Vec2::new(50.0, 50.0), Vec2::new(0.0, 50.0)),
            LineSegment::new(Vec2::new(0.0, 50.0), Vec2::new(0.0, 0.0)),
        ],
    };

    // Example with Macroquad's Rect
    let rect_object = Rect::new(200.0, 200.0, 100.0, 50.0);

    let projectile = Projectile::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));

    if let Some((distance, normal)) = projectile.check_collision(&custom_object) {
        println!("Collision with custom object detected at distance: {}, normal: {:?}", distance, normal);
    }

    if let Some((distance, normal)) = projectile.check_collision(&rect_object) {
        println!("Collision with rect object detected at distance: {}, normal: {:?}", distance, normal);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let player_sprite = include_texture!("../assets/player.png");
    let tile_map = include_texture!("../assets/map_tiles.png");
    let vehicle_sprite = include_texture!("../assets/Car-0001.png");
    let mut player = Player::new(player_sprite);
    
    let map = Map::new(tile_map, 1024);
    let mut world = World::new(map);
    let vehicle = Vehicle::new(vehicle_sprite);
    world.add_vehicle(vehicle);
    let mut controller = ControllerDirectionState::default();

    let weapon_sprite = include_texture!("../assets/pistol.png");
    let mut weapon = Weapon::new(
        weapon_sprite,
        2.,
    );
    loop {
        controller.handle_key_inputs(&mut player);
        clear_background(DARKGREEN);
        world.map.draw(player.pos, 5);

        if is_key_pressed(KeyCode::F) {
            if let Some(vehicle) = player.in_vehicle.clone() {
                player.leave_vehicle(vehicle);
                println!("left vehicle")
            } else {
                if let Some(vehicle) = world.vehicles
                    .iter()
                    .find(|v| v.borrow().position().distance(player.pos) < 100.)
                {
                    player.enter_vehicle(vehicle.clone()); // increase ref-count.
                    println!("entered vehicle");
                }
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            weapon.shoot(mouse_direction(), &world.collideables);
        }
        player.draw();
        weapon.pos = player.pos;
        weapon.pos.y += 30.;
        weapon.draw_at_world_space(player.pos);
        player.update();
        world.vehicles.iter().for_each(|v| {
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
        
        
        //pos.normalize()
        draw_text(
            &format!("pos: {:.2?}", player.pos),
            screen_width() / 2.,
            screen_height() - 40.,
            50.,
            WHITE,
        );
        draw_text(
            &format!("pos (tile): {:.2?}", world.map.to_tile_index_pos(player.pos)),
            screen_width() / 2.,
            screen_height() - 70.,
            50.,
            WHITE,
        );
        let center: Vec2 = (screen_width() / 2., screen_height() / 2.).into();
        
        draw_vector(center, mouse_direction(), 100., MAGENTA);
        println!("{:?}", mouse_direction());
        next_frame().await
    }
    
}

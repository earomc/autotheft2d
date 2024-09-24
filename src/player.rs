use std::{cell::RefCell, rc::Rc};

use crate::{draw::Draw, vehicle::Vehicle, Direction, Update};
use macroquad::prelude::*;

pub const PLAYER_TEXTURE_SCALING_FAC: f32 = 4.;
pub const PLAYER_SPRITE_SIZE: f32 = 16.;
pub const SCALED_PLAYER_SPRITE_SIZE: f32 = PLAYER_TEXTURE_SCALING_FAC * PLAYER_SPRITE_SIZE;

pub struct Player<'a> {
    pub pos: Vec2,
    pub facing: Direction,
    pub texture: &'a Texture2D,
    pub movement_speed: f32,
    pub in_vehicle: Option<Rc<RefCell<Vehicle<'a>>>>,
}

impl<'a> Player<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Player {
            pos: (0., 0.).into(),
            facing: Direction::South,
            movement_speed: 300.,
            texture,
            in_vehicle: None,
        }
    }

    pub fn pos_add(&mut self, translation: Vec2) {
        self.pos.x += translation.x.ceil();
        self.pos.y += translation.y.ceil();
    }

    pub fn draw(&self) {
        if self.in_vehicle.is_some() {
            return;
        }
        let screen_pos = (
            screen_width() / 2. - self.texture.width() / 2.,
            screen_height() / 2. - self.texture.height() / 2.,
        )
            .into();
        self.draw_at_screen_space(screen_pos);
    }

    pub fn handle_controls(&mut self, facing: Option<Direction>) {
        if let Some(vehicle) = self.in_vehicle.clone() {
            let mut vehicle = vehicle.borrow_mut();
            match facing {
                // refactor by passing in ControllerState directly? makes code simpler.
                Some(facing) => match facing {
                    Direction::North => {
                        vehicle.throttle = 1.;
                        vehicle.steer_neutral();
                    }
                    Direction::NorthEast => {
                        vehicle.throttle = 1.;
                        vehicle.steer_right();
                    }
                    Direction::East => {
                        vehicle.steer_right();
                    }
                    Direction::SouthEast => {
                        vehicle.throttle = -1.;
                        vehicle.steer_right();
                    }
                    Direction::South => {
                        vehicle.throttle = -1.;
                        vehicle.steer_neutral();
                    }
                    Direction::SouthWest => {
                        vehicle.throttle = -1.;
                        vehicle.steer_left();
                    }
                    Direction::West => {
                        vehicle.steer_left();
                    }
                    Direction::NorthWest => {
                        vehicle.throttle = 1.;
                        vehicle.steer_left();
                    }
                },
                None => {
                    vehicle.throttle = 0.;
                    vehicle.steer_neutral();
                }
            }
        } else {
            if let Some(facing) = facing {
                self.facing = facing;
                self.pos_add(facing.as_vector() * self.movement_speed * get_frame_time());
            }
        }
    }

    pub fn enter_vehicle(&mut self, vehicle: Rc<RefCell<Vehicle<'a>>>) {
        vehicle.borrow_mut().entered = true;
        self.in_vehicle = Some(vehicle)
    }

    pub fn leave_vehicle(&mut self, vehicle: Rc<RefCell<Vehicle<'a>>>) {
        let mut vehicle = vehicle.borrow_mut();
        vehicle.entered = false;
        vehicle.steer_neutral();
        vehicle.throttle = 0.;
        self.in_vehicle = None;
    }
}

impl Update for Player<'_> {
    fn update(&mut self) {
        if let Some(vehicle) = self.in_vehicle.clone() {
            self.pos = vehicle.borrow().pos;
        }
    }
}

impl<'a> Draw<'a> for Player<'a> {
    fn texture(&self) -> &'a Texture2D {
        self.texture
    }

    fn texture_size() -> f32 {
        PLAYER_SPRITE_SIZE
    }

    fn texture_size_scaled() -> f32 {
        SCALED_PLAYER_SPRITE_SIZE
    }

    fn draw_at_screen_space(&self, screen_pos: Vec2) {
        // TODO: Add dedicated sprites for all facings
        let x_texture_offset = match self.facing {
            Direction::North => 2. * PLAYER_SPRITE_SIZE,
            Direction::NorthEast => 2. * PLAYER_SPRITE_SIZE,
            Direction::East => PLAYER_SPRITE_SIZE,
            Direction::SouthEast => 0.,
            Direction::South => 0.,
            Direction::SouthWest => 0.,
            Direction::West => 3. * PLAYER_SPRITE_SIZE,
            Direction::NorthWest => 2. * PLAYER_SPRITE_SIZE,
        };
        draw_texture_ex(
            &self.texture,
            screen_pos.x,
            screen_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some((SCALED_PLAYER_SPRITE_SIZE, SCALED_PLAYER_SPRITE_SIZE).into()),
                source: Some(Rect {
                    x: x_texture_offset,
                    y: 0.,
                    w: PLAYER_SPRITE_SIZE,
                    h: PLAYER_SPRITE_SIZE,
                }),
                ..Default::default()
            },
        );
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}

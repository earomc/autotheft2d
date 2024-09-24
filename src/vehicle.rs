use core::f32;
use std::{f32::consts::*};

use macroquad::prelude::*;

use crate::{draw::Draw, player::Player, Update};

pub struct Vehicle<'a> {
    texture: &'a Texture2D,
    pub pos: Vec2,
    pub entered: bool,
    pub wheel_base: f32,
    pub turning_angle: f32,
    pub acceleration_power: f32,
    pub velocity: f32,
    pub max_velocity: f32,
    pub throttle: f32,
    pub rotation: Vec2,
}

pub const TEX_SIZE: f32 = 32.;
pub const SCALING_FAC: f32 = 8.;

impl<'a> Draw<'a> for Vehicle<'a> {
    fn texture(&self) -> &'a Texture2D {
        self.texture
    }

    fn texture_size() -> f32 {
        TEX_SIZE
    }

    fn texture_size_scaled() -> f32 {
        TEX_SIZE * SCALING_FAC
    }

    fn draw_at_screen_space(&self, screen_pos: Vec2) {
        //println!("Vehicle pos: {:?}",self.pos);

        draw_texture_ex(
            &self.texture,
            screen_pos.x,
            screen_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some((TEX_SIZE * SCALING_FAC, TEX_SIZE * SCALING_FAC).into()),
                rotation: self.rotation.to_angle() + FRAC_PI_2,
                //pivot: (),
                ..Default::default()
            },
        );
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}

impl Update for Vehicle<'_> {
    fn update(&mut self) {
        self.velocity = (get_frame_time() * self.acceleration_power * self.throttle).min(self.max_velocity);
        self.pos += self.rotation * self.velocity;
        if self.velocity > 0. {
            self.rotation = self.rotation.rotate(Vec2::from_angle(self.turning_angle * 0.03));
        }
    }
}

impl<'a> Vehicle<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Vehicle {
            texture,
            pos: (0., 0.).into(),
            entered: false,
            wheel_base: 256.,
            throttle: 0.,
            turning_angle: 0., // 45 degrees
            rotation: Vec2::new(0.,-1.).normalize(),
            acceleration_power: 1000.,
            max_velocity: 1000.,
            velocity: 0.
        }
    }
    
    pub fn steer_right(&mut self) {
        self.turning_angle = core::f32::consts::FRAC_PI_2;
    }
    
    pub fn steer_left(&mut self) {
        self.turning_angle = -core::f32::consts::FRAC_PI_2;
    }
    
    pub fn steer_neutral(&mut self) {
        self.turning_angle = 0.;
    }
}

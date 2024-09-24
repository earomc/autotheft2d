use core::f32;
use std::{f32::consts::*, ptr::eq};

use macroquad::prelude::*;

use crate::{draw::Draw, player::Player, Update};

pub struct Vehicle<'a> {
    texture: &'a Texture2D,
    pub pos: Vec2,
    pub entered: bool,
    pub acceleration: f32,
    //pub wheel_base: f32,
    pub turning_angle: f32,
    pub reversed: bool,
    pub torque: f32,
    pub reverse_torque: f32,
    pub mass: f32,
    pub velocity: f32,
    pub rotation: Vec2,
    pub throttle: f32,
    pub breaking_torque: f32,
    pub wheel_diameter: f32,
}

pub const TEX_SIZE: f32 = 32.;
pub const SCALING_FAC: f32 = 8.;
pub const DRAG_COEFFICIENT: f32 = 100.; // negative acceleration

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
        
        if self.throttle > 0. {
            self.reversed = false;
            self.apply_force(self.force_from_wheel_torque(self.throttle * self.torque), get_frame_time());
            print_velocity(self.velocity, "Accelerating!");
        } else {
            if self.velocity <= 0.0 {
                self.reversed = true;
            }
            if self.reversed {
                self.apply_force(self.force_from_wheel_torque(self.throttle * self.reverse_torque), get_frame_time());
                print_velocity(self.velocity, "Reversing!");
            } else {
                self.apply_force(self.force_from_wheel_torque(self.throttle * self.breaking_torque), get_frame_time());
                print_velocity(self.velocity, "Braking!");
            }
        }
        // F = m * a, a = F/m
        self.rotation = self
            .rotation
            .rotate(Vec2::from_angle(self.turning_angle * 0.03));
        
        let drag_force =  drag_force(
            1.293, // air density
            self.velocity,
            1.3, // reference area
            0.4 // drag coefficient
        );
        dbg!(drag_force);
        self.apply_force(-drag_force, get_frame_time());
        self.pos += self.rotation * self.velocity * 8. * if self.reversed { -1. } else { 1. };
    }
}

fn print_velocity(velocity: f32, tag: &str) {
    println!("{} {}m/s {}km/h", tag, velocity, velocity * 3.6);
}

impl<'a> Vehicle<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Vehicle {
            texture,
            acceleration: 0.,
            pos: (0., 0.).into(),
            entered: false,
            //wheel_base: 256.,
            throttle: 0.,
            turning_angle: 0., // 45 degrees
            rotation: Vec2::new(0., -1.).normalize(),
            torque: 650., // 650Nm
            reverse_torque: 400.,
            velocity: 0.,
            mass: 1300., // 1.3 metric tons
            reversed: false,
            breaking_torque: 10000.,
            wheel_diameter: 0.4
        }
    }

    pub fn steer_right(&mut self) {
        self.turning_angle = core::f32::consts::FRAC_PI_4;
    }

    pub fn steer_left(&mut self) {
        self.turning_angle = -core::f32::consts::FRAC_PI_4;
    }

    pub fn steer_neutral(&mut self) {
        self.turning_angle = 0.;
    }
    
    pub fn apply_force(&mut self, force: f32, time: f32) {
        // a = F/m
        let acceleration = force / self.mass;
        self.velocity = (self.velocity + acceleration * time).max(0.);
    }
    
    pub fn force_from_wheel_torque(&self, torque: f32) -> f32 {
        torque / self.wheel_diameter
    }
}

fn drag_force(
    mass_density: f32,
    flow_velocity: f32,
    reference_area: f32,
    drag_coefficient: f32,
) -> f32 {
    0.5 * mass_density * flow_velocity * flow_velocity * drag_coefficient * reference_area
}

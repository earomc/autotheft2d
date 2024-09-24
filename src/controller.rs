use macroquad::prelude::*;

use crate::{player::Player, Direction};

pub fn handle_key_inputs(character: &mut Player) {
    let mut state = ControllerState::default();
    state.update_state();
    if let Some(facing) = state.get_facing() {
        character.facing = facing;
        character.pos_add(facing.as_vector() * character.movement_speed * get_frame_time());
    }
}

#[derive(Default)]
pub struct ControllerState {
    w_pressed: bool,
    a_pressed: bool,
    s_pressed: bool,
    d_pressed: bool,
}

impl ControllerState {
    pub fn handle_key_inputs(&mut self, player: &mut Player) {
        self.update_state();
        player.handle_controls(self.get_facing());
    }
    fn update_state(&mut self) {
        if is_key_down(KeyCode::W) {
           self.w_pressed = true;
        } else {
            self.w_pressed = false;
        }
        if is_key_down(KeyCode::A) {
            self.a_pressed = true;
        } else {
            self.a_pressed = false;
        }
        if is_key_down(KeyCode::S) {
            self.s_pressed = true;
        } else {
            self.s_pressed = false;
        }
        if is_key_down(KeyCode::D) {
            self.d_pressed = true;
        } else {
            self.d_pressed = false;
        }
    } 
    
    fn get_facing(&self) -> Option<Direction> {
        match self {
            ControllerState {
                w_pressed: true,
                a_pressed: false,
                s_pressed: false,
                d_pressed: false,
            } => Some(Direction::North),
            ControllerState {
                w_pressed: true,
                a_pressed: false,
                s_pressed: false,
                d_pressed: true,
            } => Some(Direction::NorthEast),
            ControllerState {
                w_pressed: false,
                a_pressed: false,
                s_pressed: false,
                d_pressed: true,
            } => Some(Direction::East),
            ControllerState {
                w_pressed: false,
                a_pressed: false,
                s_pressed: true,
                d_pressed: true,
            } => Some(Direction::SouthEast),
            ControllerState {
                w_pressed: false,
                a_pressed: false,
                s_pressed: true,
                d_pressed: false,
            } => Some(Direction::South),
            ControllerState {
                w_pressed: false,
                a_pressed: true,
                s_pressed: true,
                d_pressed: false,
            } => Some(Direction::SouthWest),
            ControllerState {
                w_pressed: false,
                a_pressed: true,
                s_pressed: false,
                d_pressed: false,
            } => Some(Direction::West),
            ControllerState {
                w_pressed: true,
                a_pressed: true,
                s_pressed: false,
                d_pressed: false,
            } => Some(Direction::NorthWest),
            _ => None,
        }
    }
}

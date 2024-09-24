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
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
}

impl ControllerState {
    pub fn handle_key_inputs(&mut self, player: &mut Player) {
        self.update_state();
        player.handle_controls(self);
    }
    fn update_state(&mut self) {
        if is_key_down(KeyCode::W) {
           self.up = true;
        } else {
            self.up = false;
        }
        if is_key_down(KeyCode::A) {
            self.left = true;
        } else {
            self.left = false;
        }
        if is_key_down(KeyCode::S) {
            self.down = true;
        } else {
            self.down = false;
        }
        if is_key_down(KeyCode::D) {
            self.right = true;
        } else {
            self.right = false;
        }
    } 
    
    pub fn get_facing(&self) -> Option<Direction> {
        match self {
            ControllerState {
                up: true,
                left: false,
                down: false,
                right: false,
            } => Some(Direction::North),
            ControllerState {
                up: true,
                left: false,
                down: false,
                right: true,
            } => Some(Direction::NorthEast),
            ControllerState {
                up: false,
                left: false,
                down: false,
                right: true,
            } => Some(Direction::East),
            ControllerState {
                up: false,
                left: false,
                down: true,
                right: true,
            } => Some(Direction::SouthEast),
            ControllerState {
                up: false,
                left: false,
                down: true,
                right: false,
            } => Some(Direction::South),
            ControllerState {
                up: false,
                left: true,
                down: true,
                right: false,
            } => Some(Direction::SouthWest),
            ControllerState {
                up: false,
                left: true,
                down: false,
                right: false,
            } => Some(Direction::West),
            ControllerState {
                up: true,
                left: true,
                down: false,
                right: false,
            } => Some(Direction::NorthWest),
            _ => None,
        }
    }
}

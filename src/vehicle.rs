use macroquad::prelude::*;

use crate::draw::Draw;

pub struct Vehicle<'a> {
    texture: &'a Texture2D,
    pos: Vec2,
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
                //rotation: (),
                //pivot: (),
                ..Default::default()
            },
        );
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}

impl<'a> Vehicle<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Vehicle {
            texture,
            pos: (0., 0.).into(),
        }
    }    
}

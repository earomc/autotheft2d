use macroquad::prelude::*;

pub struct Vehicle<'a> {
    texture: &'a Texture2D,
    pos: Vec2,
}

pub const TEX_SIZE: f32 = 32.;
pub const SCALING_FAC: f32 = 8.;

impl<'a> Vehicle<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Vehicle {
            texture,
            pos: (0., 0.).into(),
        }
    }

    pub fn draw(&self, player_pos: Vec2) {
        println!("Vehicle pos: {:?}",self.pos);
        let x = -player_pos.x + self.pos.x;
        let y = -player_pos.y + self.pos.y;
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some((TEX_SIZE * SCALING_FAC, TEX_SIZE * SCALING_FAC).into()),
                //rotation: (),
                //pivot: (),
                ..Default::default()
            },
        );
    }
    
}

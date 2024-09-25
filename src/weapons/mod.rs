use macroquad::prelude::*;

use crate::draw::Draw;

pub struct Weapon<'a> {
    pub texture: &'a Texture2D,
    pub fire_cooldown_seconds: f32,
    pub pos: Vec2,
}

impl<'a> Weapon<'a> {
    pub fn new(texture: &'a Texture2D, fire_cooldown_seconds: f32) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Self {
            texture,
            fire_cooldown_seconds,
            pos: (0., 0.).into(),
        }
    }
}

impl<'a> Draw<'a> for Weapon<'a> {
    fn texture(&self) -> &'a Texture2D {
        self.texture
    }

    fn texture_size() -> f32 {
        16.
    }

    fn texture_size_scaled() -> f32 {
        16. * 4.
    }

    fn draw_at_screen_space(&self, screen_pos: Vec2) {
        draw_texture_ex(
            &self.texture,
            screen_pos.x,
            screen_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some((Self::texture_size_scaled(), Self::texture_size_scaled()).into()),
                ..Default::default()
            },
        );
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}

use macroquad::prelude::*;

pub trait Draw<'a> {
    fn texture(&self) -> &'a Texture2D;
    fn texture_size() -> f32;
    fn texture_size_scaled() -> f32;
    fn draw_at_screen_space(&self, screen_pos: Vec2);
    fn draw_at_world_space(&self, camera_pos: Vec2) {
        self.draw_at_screen_space(
            (
                -camera_pos.x + self.position().x + screen_width() / 2.
                    - Self::texture_size_scaled() / 2.,
                -camera_pos.y + self.position().y + screen_height() / 2.
                    - Self::texture_size_scaled() / 2.,
            )
                .into(),
        );
    }
    fn position(&self) -> Vec2;
}

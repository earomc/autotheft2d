use macroquad::prelude::*;
use crate::Direction;

pub const PLAYER_TEXTURE_SCALING_FAC: f32 = 4.;
pub const PLAYER_SPRITE_SIZE: f32 = 16.;
pub const SCALED_PLAYER_SPRITE_SIZE: f32 = PLAYER_TEXTURE_SCALING_FAC * PLAYER_SPRITE_SIZE;

pub struct Player {
    pub pos: Vec2,
    pub facing: Direction,
    pub texture: Texture2D,
    pub movement_speed: f32
}

impl Player {
    pub fn new(texture: Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Player {
            pos: (0., 0.).into(),
            facing: Direction::South,
            movement_speed: 300.,
            texture,
        }
    }
    
    pub fn pos_add(&mut self, translation: Vec2) {
        self.pos.x += translation.x.ceil();
        self.pos.y += translation.y.ceil();
    }

    pub fn draw(&self) {
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
            screen_width() / 2. - self.texture.width() / 2.,
            screen_height() / 2. - self.texture.height() / 2.,
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
}

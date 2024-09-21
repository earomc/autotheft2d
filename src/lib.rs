pub mod map;

use macroquad::prelude::*;

pub const CHARACTER_TEXTURE_SCALING_FAC: f32 = 4.;
pub const CHARACTER_SPRITE_SIZE: f32 = 16.;
pub const SCALED_CHARACTER_SPRITE_SIZE: f32 = CHARACTER_TEXTURE_SCALING_FAC * CHARACTER_SPRITE_SIZE;

pub const TILE_TEXTURE_SCALING_FAC: f32 = 8.;

pub struct Character {
    pub pos: (f32, f32),
    pub facing: Direction,
    pub texture: Texture2D,
}

impl Character {
    pub fn new(texture: Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Character {
            pos: (0., 0.),
            facing: Direction::SOUTH,
            texture,
        }
    }

    pub fn draw(&self) {
        let x_texture_offset = match self.facing {
            Direction::NORTH => 2. * CHARACTER_SPRITE_SIZE,
            Direction::EAST => CHARACTER_SPRITE_SIZE,
            Direction::SOUTH => 0.,
            Direction::WEST => 3. * CHARACTER_SPRITE_SIZE,
        };
        draw_texture_ex(
            &self.texture,
            screen_width() / 2. - self.texture.width() / 2.,
            screen_height() / 2. - self.texture.height() / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some((SCALED_CHARACTER_SPRITE_SIZE, SCALED_CHARACTER_SPRITE_SIZE).into()),
                source: Some(Rect {
                    x: x_texture_offset,
                    y: 0.,
                    w: CHARACTER_SPRITE_SIZE,
                    h: CHARACTER_SPRITE_SIZE,
                }),
                ..Default::default()
            },
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}
impl IntoIterator for Direction {
    type Item = Direction;

    type IntoIter = DirectionIter;

    fn into_iter(self) -> Self::IntoIter {
        DirectionIter(self)
    }
}

pub struct DirectionIter(Direction);

impl Iterator for DirectionIter {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.0 {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        };
        self.0 = next;
        Some(next)
    }
}

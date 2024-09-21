pub mod controller;
pub mod map;

use macroquad::prelude::*;

pub const CHARACTER_TEXTURE_SCALING_FAC: f32 = 4.;
pub const CHARACTER_SPRITE_SIZE: f32 = 16.;
pub const SCALED_CHARACTER_SPRITE_SIZE: f32 = CHARACTER_TEXTURE_SCALING_FAC * CHARACTER_SPRITE_SIZE;

pub const TILE_TEXTURE_SCALING_FAC: f32 = 8.;

pub struct Character {
    pub pos: Vec2,
    pub facing: Direction,
    pub texture: Texture2D,
    pub movement_speed: f32
}

impl Character {
    pub fn new(texture: Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Character {
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
            Direction::North => 2. * CHARACTER_SPRITE_SIZE,
            Direction::NorthEast => 2. * CHARACTER_SPRITE_SIZE,
            Direction::East => CHARACTER_SPRITE_SIZE,
            Direction::SouthEast => CHARACTER_SPRITE_SIZE,
            Direction::South => 0.,
            Direction::SouthWest => 0.,
            Direction::West => 3. * CHARACTER_SPRITE_SIZE,
            Direction::NorthWest => 3. * CHARACTER_SPRITE_SIZE,
            
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
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

impl Direction {
    fn as_vector(&self) -> Vec2 {
        match self {
            Direction::North => (0., -1.).into(),
            Direction::NorthEast => (45.0_f32.cos(), -45.0_f32.sin()).into(),
            Direction::East => (1., 0.).into(),
            Direction::SouthEast => (45.0_f32.cos(), 45.0_f32.sin()).into(),
            Direction::South => (0., 1.).into(),
            Direction::SouthWest => (-45.0_f32.cos(), 45.0_f32.sin()).into(),
            Direction::West => (-1., 0.).into(),
            Direction::NorthWest => (-45.0_f32.cos(), -45.0_f32.sin()).into(),
        }
    }
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
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        };
        self.0 = next;
        Some(next)
    }
}

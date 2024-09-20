use macroquad::prelude::*;

pub struct Character {
    pub facing: Direction,
    pub texture: Texture2D,
}

impl Character {
    pub fn new(texture: Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Character {
            facing: Direction::SOUTH,
            texture,
        }
    }

    pub fn draw(&self) {
        const SPRITE_SIZE: f32 = 16.;
        let x_texture_offset = match self.facing {
            Direction::NORTH => 2. * SPRITE_SIZE,
            Direction::EAST => SPRITE_SIZE,
            Direction::SOUTH => 0.,
            Direction::WEST => 3. * SPRITE_SIZE,
        };
        draw_texture_ex(
            &self.texture,
            screen_width() / 2. - self.texture.width() / 2.,
            screen_height() / 2. - self.texture.height() / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some((64., 64.).into()),
                source: Some(Rect {
                    x: x_texture_offset,
                    y: 0.,
                    w: SPRITE_SIZE,
                    h: SPRITE_SIZE,
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

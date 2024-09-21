use macroquad::prelude::*;

pub struct Map<'a> {
    tiles: Vec<Tile<'a>>,
}

impl<'a> Map<'a> {
    fn add_tile(&mut self, x: i32, y: i32, tile: Tile) {}
}

pub const TILE_TEX_SIZE: f32 = 32.;

pub struct Tile<'a> {
    texture: &'a Texture2D,
    texture_x_offset: f32
}

pub struct TileVariant {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl TileVariant {
    fn get_x_texture_offset(&self) -> f32 {
        match self {
            TileVariant { north: true, east: false, south: true, west: false } => 0.,
            TileVariant { north: false, east: true, south: true, west: false } => TILE_TEX_SIZE,
            TileVariant { north: false, east: false, south: true, west: true } => TILE_TEX_SIZE * 2.,
            TileVariant { north: true, east: true, south: false, west: false } => TILE_TEX_SIZE * 3.,
            TileVariant { north: true, east: false, south: false, west: true } => TILE_TEX_SIZE * 4.,
            TileVariant { north: true, east: true, south: true, west: true } => TILE_TEX_SIZE * 5.,
            TileVariant { north: true, east: false, south: true, west: true } => TILE_TEX_SIZE * 6.,
            TileVariant { north: true, east: true, south: true, west: false } => TILE_TEX_SIZE * 7.,
            TileVariant { north: false, east: true, south: true, west: true } => TILE_TEX_SIZE * 8.,
            TileVariant { north: true, east: true, south: false, west: true } => TILE_TEX_SIZE * 9.,
            _ => panic!("Nonexistent tile variant")
        }
    }
}

impl From<(bool, bool, bool, bool)> for TileVariant {
    fn from(value: (bool, bool, bool, bool)) -> Self {
        TileVariant {
            north: value.0,
            east: value.1,
            south: value.2,
            west: value.3,
        }
    }
}

impl<'a> Tile<'a> {
    pub fn new(texture: &'a Texture2D, variant: TileVariant) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Tile {
            texture,
            texture_x_offset: variant.get_x_texture_offset()
        }
    }
    pub fn draw(&self) {
        let x = 0.;
        let y = 0.;
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some((256., 256.).into()),
                source: Some(Rect { x: self.texture_x_offset, y: 0., w: TILE_TEX_SIZE, h: TILE_TEX_SIZE }),
                ..Default::default()
            },
        );
    }
}

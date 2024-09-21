use array2d::Array2D;
use macroquad::prelude::*;

use crate::TILE_TEXTURE_SCALING_FAC;

pub struct Map<'a> {
    tiles: Array2D<Tile<'a>>,
}

impl<'a> Map<'a> {
    pub fn new(tile_map_texture: &'a Texture2D) -> Self {
        let tile = Tile::new(tile_map_texture, TileVariant::empty());
        let mut tiles = Array2D::filled_with(tile, 16, 16);
        for y in 0..16 {
            tiles[(8, y)] =  Tile::new(tile_map_texture, (true, false, true, false).into());
        }
        Map { tiles }
    }

    pub fn draw(&self, world_space_pos: (f32, f32)) {
        for (tile_x_worldspace, tile_row) in self.tiles.rows_iter().enumerate() {
            for (tile_y_worldspace, tile) in tile_row.enumerate() {
                tile.draw_at_screenspace(
                    //  + (screen_width() / 2. - TILE_TEX_SIZE / 2.)
                    // + (screen_height() / 2. - TILE_TEX_SIZE / 2.)
                    -world_space_pos.0
                        + tile_x_worldspace as f32 * TILE_TEX_SIZE * TILE_TEXTURE_SCALING_FAC,
                    -world_space_pos.1
                        + tile_y_worldspace as f32 * TILE_TEX_SIZE * TILE_TEXTURE_SCALING_FAC,
                );
            }
        }
    }
    fn add_tile(&mut self, x: i32, y: i32, tile: Tile) {}
}

pub const TILE_TEX_SIZE: f32 = 32.;

#[derive(Clone)]
pub struct Tile<'a> {
    texture: &'a Texture2D,
    texture_x_offset: f32,
}

pub struct TileVariant {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl TileVariant {
    fn empty() -> Self {
        (false, false, false, false).into()
    }
    fn full_crossing() -> Self {
        (true, true, true, true).into()
    }
    fn get_x_texture_offset(&self) -> f32 {
        match self {
            TileVariant {
                north: true,
                east: false,
                south: true,
                west: false,
            } => 0.,
            TileVariant {
                north: false,
                east: true,
                south: false,
                west: true,
            } => TILE_TEX_SIZE,
            TileVariant {
                north: false,
                east: true,
                south: true,
                west: false,
            } => TILE_TEX_SIZE * 2.,
            TileVariant {
                north: false,
                east: false,
                south: true,
                west: true,
            } => TILE_TEX_SIZE * 3.,
            TileVariant {
                north: true,
                east: true,
                south: false,
                west: false,
            } => TILE_TEX_SIZE * 4.,
            TileVariant {
                north: true,
                east: false,
                south: false,
                west: true,
            } => TILE_TEX_SIZE * 5.,
            TileVariant {
                north: true,
                east: true,
                south: true,
                west: true,
            } => TILE_TEX_SIZE * 6.,
            TileVariant {
                north: true,
                east: false,
                south: true,
                west: true,
            } => TILE_TEX_SIZE * 7.,
            TileVariant {
                north: true,
                east: true,
                south: true,
                west: false,
            } => TILE_TEX_SIZE * 8.,
            TileVariant {
                north: false,
                east: true,
                south: true,
                west: true,
            } => TILE_TEX_SIZE * 9.,
            TileVariant {
                north: true,
                east: true,
                south: false,
                west: true,
            } => TILE_TEX_SIZE * 10.,
            TileVariant {
                north: false,
                east: false,
                south: false,
                west: false,
            } => TILE_TEX_SIZE * 11.,
            _ => panic!("Nonexistent tile variant"),
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
            texture_x_offset: variant.get_x_texture_offset(),
        }
    }

    pub fn draw_at_screenspace(&self, x: f32, y: f32) {
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect {
                    x: self.texture_x_offset,
                    y: 0.,
                    w: TILE_TEX_SIZE,
                    h: TILE_TEX_SIZE,
                }),
                dest_size: Some(
                    (
                        TILE_TEX_SIZE * TILE_TEXTURE_SCALING_FAC,
                        TILE_TEX_SIZE * TILE_TEXTURE_SCALING_FAC,
                    )
                        .into(),
                ),
                ..Default::default()
            },
        );
    }
}

use macroquad::prelude::*;
use ndarray::Array2;

use crate::{draw::Draw, util::SpiralIterator};

pub const TILE_TEX_SIZE: f32 = 32.;
pub const TILE_TEXTURE_SCALING_FAC: f32 = 16.;
pub const TILE_TEX_SIZE_SCALED: f32 = TILE_TEX_SIZE * TILE_TEXTURE_SCALING_FAC;

pub struct Map {
    tiles: Array2<Tile>,
    world_size: usize,
}

impl Map {
    pub fn new(tile_map_texture: Texture2D, world_length_tiles: usize) -> Self {
        let tile = Tile::new(
            tile_map_texture,
            (0., 0.).into(),
            (true, true, true, true).into(),
        );
        let mut tiles = Array2::from_elem((world_length_tiles, world_length_tiles), tile);
        for ((x_tile, y_tile), tile) in tiles.indexed_iter_mut() {
            let x_tile = x_tile as i32 - world_length_tiles as i32 / 2;
            let y_tile = y_tile as i32 - world_length_tiles as i32 / 2;
            tile.pos = (
                x_tile as f32 * Tile::texture_size_scaled(),
                y_tile as f32 * Tile::texture_size_scaled(),
            )
                .into();
        }
        Map { tiles, world_size: world_length_tiles }
    }
    
    // tpos_world = (tpos - world_len / 2) * tex_scaled
    // tpos = tpos_world / tex_scaled + world_len / 2
    pub fn to_tile_index_pos(&self, world_pos: Vec2) -> (usize, usize) {
        let x = (world_pos.x / Tile::texture_size_scaled()) as isize + self.world_size as isize / 2;
        let y = (world_pos.y / Tile::texture_size_scaled()) as isize + self.world_size as isize / 2;
        (x as usize, y as usize)
    }
    
    pub fn draw(&self, player_pos: Vec2, radius: usize) -> Option<&Tile> {
        let center = self.to_tile_index_pos(player_pos);
        let spiral = SpiralIterator::new(center);
        self.tiles.get(center).map(|tile| tile.draw_at_world_space(player_pos));
        spiral.take(radius * radius).for_each(|pos| {
            self.tiles.get(pos).map(|tile| tile.draw_at_world_space(player_pos));
        });
        
        self.tiles.get(self.to_tile_index_pos(player_pos))
    }
    
    pub fn get_tile(&self, pos: (usize, usize)) -> Option<&Tile> {
        self.tiles.get(pos)
    }
}

#[derive(Clone)]
pub struct Tile {
    pos: Vec2,
    texture: Texture2D,
    texture_x_offset: f32,
}

impl Tile {
    pub fn new(texture: Texture2D, pos: Vec2, variant: TileVariant) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Tile {
            texture,
            texture_x_offset: variant.get_x_texture_offset(),
            pos,
        }
    }
}

impl Draw for Tile {
    fn texture(&self) -> &Texture2D {
        &self.texture
    }

    fn texture_size() -> f32 {
        TILE_TEX_SIZE
    }

    fn texture_size_scaled() -> f32 {
        TILE_TEX_SIZE_SCALED
    }

    fn draw_at_screen_space(&self, pos: Vec2) {
        draw_texture_ex(
            &self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect {
                    x: self.texture_x_offset,
                    y: 0.,
                    w: Tile::texture_size(),
                    h: Tile::texture_size(),
                }),
                dest_size: Some((Tile::texture_size_scaled(), Tile::texture_size_scaled()).into()),
                ..Default::default()
            },
        );
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
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

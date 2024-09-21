use macroquad::texture::Texture2D;

pub struct Map<'a> {
    tiles: Vec<Tile<'a>>
}

impl<'a> Map<'a> {
    fn add_tile(&mut self, x: i32, y: i32, tile: Tile) {
        
    }
}


pub struct Tile<'a> {
    map_position: (i32, i32), // tile map coordinate coordinate
    texture: &'a Texture2D
}

impl<'a> Tile<'a> {
    pub fn new(map_position: (i32, i32), texture: &'a Texture2D) -> Self {
        Tile { map_position, texture }
    }
    pub fn draw_tile(&self) {}
}
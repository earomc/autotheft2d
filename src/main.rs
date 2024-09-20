use macroquad::{prelude::*, telemetry::frame};
use macroquad_tiled as tiled;

const WINDOW_HEIGHT: i32 = 720;
const WINDOW_WIDTH: i32 = 1280;

fn window_conf() -> Conf {
    Conf {
        window_title: "AutoTheft2D".into(),
        window_resizable: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let character_sprite = load_texture("assets/character.png").await.unwrap();
    let mut character = Character::new(character_sprite);
    let mut direction_iter = Direction::NORTH.into_iter();
    let mut timer_current = 0.;
    loop {
        timer_current += get_frame_time();
        if timer_current >= 0.5 {
            character.facing = direction_iter.next().unwrap();
            println!("Advanced to {:?} ", character.facing);
            timer_current -= 0.5
        }
        character.draw();
        next_frame().await
    }
}

struct Character {
    facing: Direction,
    texture: Texture2D
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}
impl IntoIterator for Direction {
    type Item = Direction;

    type IntoIter = DirectionIter;

    fn into_iter(self) -> Self::IntoIter {
        DirectionIter(self)
    }
}

struct DirectionIter(Direction);

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

impl Character {
    fn new(texture: Texture2D) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Character { facing: Direction::SOUTH, texture }
    }
    
    pub fn draw(&self) {
        let x_texture_offset = match self.facing {
            Direction::NORTH => 2. * 16.,
            Direction::EAST => 16.,
            Direction::SOUTH => 0.,
            Direction::WEST => 3. * 16.,
        };
        draw_texture_ex(
            &self.texture,
            screen_width() / 2. - self.texture.width() / 2.,
            screen_height() / 2. - self.texture.height() / 2.,
            WHITE,
            DrawTextureParams { dest_size: Some((64., 64.).into()), source: Some(Rect { x: x_texture_offset, y: 0., w: 16., h: 16. }), ..Default::default() }
        );
    }
}

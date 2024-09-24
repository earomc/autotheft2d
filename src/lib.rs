pub mod controller;
pub mod draw;
pub mod map;
pub mod player;
pub mod vehicle;

use macroquad::prelude::*;

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

pub trait Update {
    fn update(&mut self);
}

use std::{cell::RefCell, rc::Rc};

use macroquad::prelude::*;

use crate::{collide::Collide, draw::Draw, vehicle::{self, Vehicle}};

pub struct Weapon {
    pub texture: Texture2D,
    pub fire_cooldown_seconds: f32,
    pub pos: Vec2,
}

impl Weapon {
    pub fn new(texture: Texture2D, fire_cooldown_seconds: f32) -> Self {
        texture.set_filter(FilterMode::Nearest);
        Self {
            texture,
            fire_cooldown_seconds,
            pos: (0., 0.).into(),
        }
    }

    pub fn shoot(&self, direction: Vec2, objects: &[Rc<RefCell<(dyn Collide + 'static)>>]) -> Option<Rc<RefCell<dyn Collide>>> {
        let projectile = Projectile::new(self.pos, direction);
        for object in objects {
            let object = object.borrow();
            if let Some((t1, normal)) = projectile.check_collision(&*object) {
                println!("Collision");
            }
        }
        None
    }
}

impl Draw for Weapon {
    fn texture(&self) -> &Texture2D {
        &self.texture
    }

    fn texture_size() -> f32 {
        16.
    }

    fn texture_size_scaled() -> f32 {
        16. * 4.
    }

    fn draw_at_screen_space(&self, screen_pos: Vec2) {
        draw_texture_ex(
            &self.texture,
            screen_pos.x,
            screen_pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some((Self::texture_size_scaled(), Self::texture_size_scaled()).into()),
                ..Default::default()
            },
        );
    }

    fn position(&self) -> Vec2 {
        self.pos
    }
}


// Projectile struct for 2D
pub struct Projectile {
    pub origin: Vec2,
    pub direction: Vec2,
}

impl Projectile {
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn check_collision(&self, object: &dyn Collide) -> Option<(f32, Vec2)> {
        object.collides_ray(self.origin, self.direction)
    }
}

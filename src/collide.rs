use macroquad::prelude::*;

pub trait Collide {
    fn collision_shape(&self) -> Vec<LineSegment>;
    
    fn collides_ray(&self, ray_origin: Vec2, ray_direction: Vec2) -> Option<(f32, Vec2)> {
        let mut closest_hit: Option<(f32, Vec2)> = None;
        
        for segment in self.collision_shape().iter() {
            if let Some((t, normal)) = segment.intersect_ray(ray_origin, ray_direction) {
                if closest_hit.is_none() || t < closest_hit.unwrap().0 {
                    closest_hit = Some((t, normal));
                }
            }
        }
        
        closest_hit
    }
}

#[derive(Clone, Copy)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
}

impl LineSegment {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }

    /// Checks if the given ray intersects with this line segment.
    /// Returns Some(t1, normal)
    /// t1 represents the distance along the ray from the ray origin to the intersection point, expressed as a multiple of the ray's direction vector.
    /// If t1 is 0, the intersection is at the ray's origin.
    /// If t1 is 1, the intersection is one full length of the direction vector away from the origin.
    /// If t1 is negative, the intersection is behind the ray's origin.
    /// If t1 is positive and finite, it tells us how far along the ray the intersection occurs.
    /// None if the ray does not intersect
    pub fn intersect_ray(&self, ray_origin: Vec2, ray_direction: Vec2) -> Option<(f32, Vec2)> {
        let v1 = ray_origin - self.start;
        let v2 = self.end - self.start;
        let v3 = Vec2::new(-ray_direction.y, ray_direction.x);

        let dot = v2.dot(v3);
        if dot.abs() < 1e-6 {
            return None; // The line and ray are parallel
        }

        let t1 = v2.perp_dot(v1) / dot;
        let t2 = v1.dot(v3) / dot;

        if t1 >= 0.0 && (0.0..=1.0).contains(&t2) {
            let normal = Vec2::new(v2.y, -v2.x).normalize();
            Some((t1, normal))
        } else {
            None
        }
    }
}

// Implementation for Macroquad's Rect struct
impl Collide for Rect {
    fn collision_shape(&self) -> Vec<LineSegment> {
        let top_left = Vec2::new(self.x, self.y);
        let top_right = Vec2::new(self.x + self.w, self.y);
        let bottom_right = Vec2::new(self.x + self.w, self.y + self.h);
        let bottom_left = Vec2::new(self.x, self.y + self.h);

        vec![
            LineSegment::new(top_left, top_right),
            LineSegment::new(top_right, bottom_right),
            LineSegment::new(bottom_right, bottom_left),
            LineSegment::new(bottom_left, top_left),
        ]
    }
}

pub struct Object2D {
    pub position: Vec2,
    pub shape: Vec<LineSegment>,
}

impl Collide for Object2D {
    fn collision_shape(&self) -> Vec<LineSegment> {
        self.shape.iter().map(|s| LineSegment::new(
            s.start + self.position,
            s.end + self.position
        )).collect()
    }
}
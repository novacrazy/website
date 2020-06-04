use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);

    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
}

impl Vector2 {
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn norm_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(mut self, rhs: Self) -> Vector2 {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(mut self, rhs: f32) -> Vector2 {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

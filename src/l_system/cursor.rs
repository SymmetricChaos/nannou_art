use nannou::{math::Vec2Rotate, prelude::Vec2};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cursor {
    position: Vec2,
    angle: Vec2,
}

const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;

impl Cursor {
    pub fn new(position: impl Into<Vec2>, angle: impl Into<Vec2>) -> Self {
        Cursor {
            position: Into::into(position),
            angle: Into::into(angle)
                .try_normalize()
                .expect("unable to normalize angle"),
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn angle(&self) -> Vec2 {
        self.angle
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position
    }

    pub fn set_angle(&mut self, angle: Vec2) {
        self.angle = angle.try_normalize().expect("unable to normalize angle")
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle = self.angle.rotate(radians)
    }

    pub fn rotate_degrees(&mut self, degrees: f32) {
        self.angle = self.angle.rotate(degrees * DEG_TO_RAD)
    }

    pub fn forward(&mut self, distance: f32) {
        self.position += self.angle * distance
    }
}

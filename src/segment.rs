use nannou::{
    draw::{primitive::Line, Drawing},
    prelude::Vec2,
    Draw,
};

#[derive(Debug, Copy, Clone)]
pub struct Segment {
    start: Vec2,
    end: Vec2,
}

impl Segment {
    pub fn scaled(mut self, scale: f32) -> Self {
        self.start *= scale;
        self.end *= scale;
        self
    }

    pub fn offset(mut self, offset: Vec2) -> Self {
        self.start -= offset;
        self.end -= offset;
        self
    }

    pub fn line<'a>(&'a self, draw: &'a Draw) -> Drawing<Line> {
        draw.line().start(self.start).end(self.end)
    }

    pub fn center(&self) -> Vec2 {
        (self.start + self.end) / 2.0
    }
}

impl From<((i32, i32), (i32, i32))> for Segment {
    fn from(value: ((i32, i32), (i32, i32))) -> Self {
        let start = (value.0 .0 as f32, value.0 .1 as f32).into();
        let end = (value.1 .0 as f32, value.1 .1 as f32).into();
        Self { start, end }
    }
}

impl From<(Vec2, Vec2)> for Segment {
    fn from(value: (Vec2, Vec2)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

use itertools::Itertools;
use itertools_num::linspace;
use nannou::{
    math::Vec2Rotate,
    prelude::{Hsv, Update, Vec2, BLACK},
    App, Frame,
};

use crate::helper::Segment;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cursor {
    position: Vec2,
    angle: Vec2,
}

impl Cursor {
    pub fn new(position: impl Into<Vec2>, angle: impl Into<Vec2>) -> Self {
        Cursor {
            position: Into::into(position),
            angle: Into::into(angle).normalize(),
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
        self.angle = angle
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle = self.angle.rotate(radians)
    }

    pub fn forward(&mut self, distance: f32) {
        self.position += self.angle * distance
    }
}

pub struct Model {
    expression: Vec<char>,
    segments: Vec<Segment>,
    cursor: Cursor,
}

const DEPTH: usize = 5;

pub fn model(_app: &App) -> Model {
    let mut expression = String::from("X");
    for _ in 0..DEPTH {
        let mut new = String::new();
        for c in expression.chars() {
            match c {
                'X' => new.push_str("X+YF++YF-FX--FXFX-YF+"),
                'Y' => new.push_str("-FX+YFYF++YF+FX--FX-Y"),
                _ => new.push(c),
            }
        }
        expression = new;
    }

    let cursor = Cursor::new((0.0, 0.0), (1.0, 1.0));

    Model {
        expression: expression.chars().collect_vec(),
        segments: Vec::new(),
        cursor,
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    loop {
        if let Some(c) = model.expression.pop() {
            let mut new_cursor = model.cursor;
            match c {
                'F' => {
                    new_cursor.forward(10.0);
                    model
                        .segments
                        .push(Segment::from((model.cursor.position, new_cursor.position)));
                    model.cursor = new_cursor;
                    break;
                }
                '+' => model.cursor.rotate(1.0472),
                '-' => model.cursor.rotate(-1.0472),
                _ => (),
            }
        } else {
            app.quit();
            break;
        }
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let hue = linspace(-180.0_f32, 120.0, 2_usize.pow((2 * DEPTH) as u32))
        .map(|degree| Hsv::new(degree, 1.0, 0.5));

    for (segment, hue) in model.segments.iter().zip(hue) {
        segment.line(&draw).color(hue).weight(5.0).caps_round();
    }

    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p hilbert_curve.mp4

    // use crate::helper::captured_frame_path;
    // let file_path = captured_frame_path(app, &frame, "hilbert_curve");
    // app.main_window().capture_frame(file_path);
}
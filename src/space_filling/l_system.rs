use std::collections::HashMap;

use itertools::Itertools;
use nannou::{
    math::Vec2Rotate,
    prelude::{Update, Vec2, BLACK, SALMON},
    App, Frame,
};

use crate::segment::Segment;

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

pub fn l_system(axiom: String, rules: HashMap<char, &str>, depth: usize) -> Vec<char> {
    let mut expression = axiom;
    for _ in 0..depth {
        let mut new = String::new();
        for c in expression.chars() {
            if let Some(s) = rules.get(&c) {
                new.push_str(s)
            } else {
                new.push(c)
            }
        }
        expression = new;
    }
    expression.chars().collect_vec()
}

pub struct Model {
    expression: Vec<char>,
    segments: Vec<Segment>,
    angle: f32,
    cursor: Cursor,
}

pub fn model(_app: &App) -> Model {
    // let expression = l_system(
    //     String::from("X"),
    //     HashMap::from([
    //         ('X', "X+YF++YF-FX--FXFX-YF+"),
    //         ('Y', "-FX+YFYF++YF+FX--FX-Y"),
    //     ]),
    //     5,
    // );

    let expression = l_system(String::from("F"), HashMap::from([('F', "F+F-F-F+F")]), 4);

    let cursor = Cursor::new((354.72446, -550.3741), (-1.0, 1.0));
    // let angle = 1.0471975512; // 60 degrees
    let angle = 1.57079632679; // 90 degrees

    Model {
        expression,
        segments: Vec::new(),
        angle,
        cursor,
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    if app.keys.down.contains(&nannou::prelude::Key::A) {
        let x = {
            let mut x = model.segments.iter().map(|s| s.center().x).sum::<f32>();
            x /= model.segments.len() as f32;
            x -= model.cursor.position.x;
            x
        };
        let y = {
            let mut y = model.segments.iter().map(|s| s.center().y).sum::<f32>();
            y /= model.segments.len() as f32;
            y -= model.cursor.position.y;
            y
        };
        println!("center: ({x},{y})");
    }
    loop {
        if let Some(c) = model.expression.pop() {
            let mut new_cursor = model.cursor;
            match c {
                'F' => {
                    new_cursor.forward(5.0);
                    model
                        .segments
                        .push(Segment::from((model.cursor.position, new_cursor.position)));
                    model.cursor = new_cursor;
                    break;
                }
                '+' => model.cursor.rotate(model.angle),
                '-' => model.cursor.rotate(-model.angle),
                _ => (),
            }
        } else {
            //app.quit();
            break;
        }
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for segment in model.segments.iter() {
        segment.line(&draw).color(SALMON).weight(2.0).caps_round();
    }

    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p hilbert_curve.mp4

    // use crate::capture::captured_frame_path;
    // let file_path = captured_frame_path(app, &frame, "hilbert_curve");
    // app.main_window().capture_frame(file_path);
}

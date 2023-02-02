use itertools::Itertools;
use itertools_num::linspace;
use nannou::{
    prelude::{Hsv, Update, Vec2, BLACK},
    App, Frame,
};

use crate::segment::Segment;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_vec2(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::from((0.0, 1.0)),
            Direction::Down => Vec2::from((0.0, -1.0)),
            Direction::Left => Vec2::from((-1.0, 0.0)),
            Direction::Right => Vec2::from((1.0, 0.0)),
        }
    }

    pub fn turn_left(&mut self) {
        match self {
            Direction::Up => *self = Direction::Left,
            Direction::Down => *self = Direction::Right,
            Direction::Left => *self = Direction::Down,
            Direction::Right => *self = Direction::Up,
        }
    }

    pub fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
            Direction::Right => *self = Direction::Down,
        }
    }
}

pub struct Model {
    expression: Vec<char>,
    segments: Vec<Segment>,
    cursor: Vec2,
    direction: Direction,
    scale: f32,
}

const DEPTH: usize = 6;

pub fn model(_app: &App) -> Model {
    let mut expression = String::from("A");
    for _ in 0..DEPTH {
        let mut new = String::new();
        for c in expression.chars() {
            match c {
                'A' => new.push_str("+BF-AFA-FB+"),
                'B' => new.push_str("-AF+BFB+FA-"),
                _ => new.push(c),
            }
        }
        expression = new;
    }

    let cursor = {
        let mut pos = Vec2::from((1.0, 1.0));
        for _ in 1..DEPTH {
            pos *= 2.0;
            pos += 1.0;
        }
        -pos / 2.0
    };

    Model {
        expression: expression.chars().collect_vec(),
        segments: Vec::new(),
        cursor,
        direction: Direction::Right,
        scale: 10.0,
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    loop {
        if let Some(c) = model.expression.pop() {
            let mut new_cursor = model.cursor;
            match c {
                'F' => {
                    new_cursor += model.direction.as_vec2();
                    model
                        .segments
                        .push(Segment::from((model.cursor, new_cursor)).scaled(model.scale));
                    model.cursor = new_cursor;
                    break;
                }
                '+' => model.direction.turn_left(),
                '-' => model.direction.turn_right(),
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

    use crate::capture::captured_frame_path;
    let file_path = captured_frame_path(app, &frame, "hilbert_curve");
    app.main_window().capture_frame(file_path);
}

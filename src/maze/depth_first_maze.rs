use itertools::{iproduct, Itertools};
use nannou::{
    prelude::*,
    rand::{seq::SliceRandom, thread_rng},
};

use crate::helper::Segment;

pub struct Model {
    segments: Vec<Segment>,
    finished_segments: Vec<Segment>,
    stack: Vec<(i32, i32)>,
    cursor: (i32, i32),
    scale: f32,
    cells: Vec<(i32, i32)>,
}

impl Model {
    fn outline(&self, draw: &Draw) {
        let r_size = (WIDTH + 1) as f32 * self.scale * 2.0;
        draw.rect()
            .no_fill()
            .w_h(r_size, r_size)
            .stroke_color(BLACK)
            .stroke_weight(3.0);
    }

    pub fn neighbors_of(&self, p: (i32, i32)) -> Vec<(i32, i32)> {
        [
            (p.0 - 1, p.1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
        ]
        .iter()
        .filter(|x| self.cells.contains(x))
        .copied()
        .collect_vec()
    }

    fn neighbors(&self) -> Vec<(i32, i32)> {
        self.neighbors_of(self.cursor)
    }
}

const WIDTH: i32 = 10;

pub fn model(_app: &App) -> Model {
    let cells = {
        let xs = -WIDTH..=WIDTH;
        let ys = -WIDTH..=WIDTH;
        iproduct!(xs, ys).into_iter().collect_vec()
    };

    Model {
        segments: Vec::new(),
        finished_segments: Vec::new(),
        stack: Vec::new(),
        cursor: (0, 0),
        scale: 20.0,
        cells,
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    // use std::{thread, time};

    // let t = time::Duration::from_millis(50);
    // thread::sleep(t);

    if model.cells.is_empty() {
        if model.segments.is_empty() {
            app.quit();
        } else {
            model.finished_segments.push(model.segments.pop().unwrap());
        }
    } else {
        let mut rng = thread_rng();
        let neighbors = model.neighbors();
        // If there are no neighbors backtrack
        if neighbors.is_empty() {
            model.cursor = model.stack.pop().unwrap();
            model.finished_segments.push(model.segments.pop().unwrap());
        // If there are neighbors pick one, move to cursor there
        } else {
            model.stack.push(model.cursor);
            let new_pos = *neighbors.choose(&mut rng).unwrap();
            model.cells.retain(|x| x != &new_pos);
            model
                .segments
                .push(Segment::from((model.cursor, new_pos)).scaled(model.scale));
            model.cursor = new_pos;
        }
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PALEGOLDENROD);
    for segment in model.segments.iter() {
        segment
            .line(&draw)
            .color(SLATEBLUE)
            .weight(2.0)
            .caps_round();
    }
    for segment in model.finished_segments.iter() {
        segment.line(&draw).color(BLACK).weight(3.0).caps_round();
    }

    model.outline(&draw);
    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p depth_first_maze.mp4

    // use crate::helper::captured_frame_path;
    // let file_path = captured_frame_path(app, &frame, "depth_first_maze");
    // app.main_window().capture_frame(file_path);
}

use itertools::{iproduct, Itertools};
use nannou::{
    draw::{primitive::Line, Drawing},
    prelude::*,
    rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng},
};

pub struct Cells {
    cells: Vec<(i32, i32)>,
}

impl Cells {
    pub fn neighbors(&self, p: (i32, i32)) -> Vec<(i32, i32)> {
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
}

pub struct Segment {
    start: Vec2,
    end: Vec2,
}

impl Segment {
    fn scaled(mut self, scale: f32) -> Self {
        self.start *= scale;
        self.end *= scale;
        self
    }

    fn line<'a>(&'a self, draw: &'a Draw) -> Drawing<Line> {
        draw.line().start(self.start).end(self.end)
    }
}

impl From<((i32, i32), (i32, i32))> for Segment {
    fn from(value: ((i32, i32), (i32, i32))) -> Self {
        let start = (value.0 .0 as f32, value.0 .1 as f32);
        let end = (value.1 .0 as f32, value.1 .1 as f32);
        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}

pub struct Model {
    segments: Vec<Segment>,
    active: Vec<(i32, i32)>,
    cursor: (i32, i32),
    scale: f32,
    cells: Cells,
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

    fn move_cursor_to_random(&mut self, rng: &mut ThreadRng) {
        if let Some(cell) = self.active.choose(rng) {
            self.cursor = *cell;
        }
    }

    fn neighbors(&self) -> Vec<(i32, i32)> {
        self.cells.neighbors(self.cursor)
    }
}

const WIDTH: i32 = 10;

pub fn model(_app: &App) -> Model {
    let cells = {
        let xs = -WIDTH..=WIDTH;
        let ys = -WIDTH..=WIDTH;
        Cells {
            cells: iproduct!(xs, ys).into_iter().collect_vec(),
        }
    };

    Model {
        segments: Vec::new(),
        active: vec![(0, 0)],
        cursor: (0, 0),
        scale: 20.0,
        cells,
    }
}

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let mut rng = thread_rng();
    let mut neighbors = model.neighbors();

    if model.active.is_empty() {
        app.quit()
    }

    // If there are no neighbors the cell is dead so remove it and move to a random active cell
    while neighbors.is_empty() {
        model.active.retain(|x| x != &model.cursor);
        if model.active.is_empty() {
            break;
        }
        // Move the cursor to a random active position
        model.move_cursor_to_random(&mut rng);
        neighbors = model.neighbors();
    }

    if model.active.is_empty() {
        app.quit();
    } else {
        // If there are neighbors pick one and draw the line
        let endpoint = *neighbors.choose(&mut rng).unwrap();
        model
            .segments
            .push(Segment::from((model.cursor, endpoint)).scaled(model.scale));

        // Mark the endpoint as active and remove it from cells
        model.active.push(endpoint);
        model.cells.cells.retain(|x| x != &endpoint);

        // Move the cursor to a random active position
        model.move_cursor_to_random(&mut rng);
    }
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PALEGOLDENROD);
    model.outline(&draw);

    for segment in model.segments.iter() {
        segment.line(&draw).color(BLACK).weight(3.0).caps_round();
    }

    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p breadth_first_maze.mp4

    use crate::helper::captured_frame_path;
    let file_path = captured_frame_path(app, &frame, "breadth_first_maze");
    app.main_window().capture_frame(file_path);
}

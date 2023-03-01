use std::collections::HashMap;

use lindenmayer::LSystem;
use nannou::{prelude::BLACK, App, Frame};

use super::{cursor::Cursor, Action, SymbolReader};

use lazy_static::lazy_static;

lazy_static! {
    static ref SYSTEM: LSystem = LSystem::new(String::from("X"), &[('X', "F[X][+FX]-FX")]);
}

pub fn model(_app: &App) -> SymbolReader {
    let actions = HashMap::from([
        ('X', Action::None),
        ('D', Action::PushPosition),
        ('F', Action::DrawForward(60.0)),
        ('+', Action::RotateDeg(-25.0)),
        ('-', Action::RotateDeg(25.0)),
        ('[', Action::PushCursor),
        (']', Action::PopCursor),
    ]);

    let cursor = Cursor::new((0.0, -200.0), (0.0, 1.0));

    let builder = SYSTEM.builder(4);

    SymbolReader::new(Box::new(builder), actions, cursor)
}

pub fn view(app: &App, model: &SymbolReader, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for segment in model.segments.iter() {
        segment
            .line(&draw)
            .rgba(0.5, 0.9, 0.2, 0.2)
            .weight(5.0)
            .caps_round();
    }

    for dot in model.positions.iter() {
        draw.ellipse().xy(*dot).radius(3.0).rgba(0.9, 0.2, 0.2, 0.2);
    }

    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p tree.mp4

    // use crate::capture::captured_frame_path;
    // let file_path = captured_frame_path(app, &frame, "tree");
    // app.main_window().capture_frame(file_path);
}

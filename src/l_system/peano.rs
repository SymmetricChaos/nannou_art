use std::collections::HashMap;

use lindenmayer::LSystem;
use nannou::{prelude::BLACK, App, Frame};

use super::{cursor::Cursor, Action, SymbolReader};

use lazy_static::lazy_static;

lazy_static! {
    // A-curve: SS-S-SS+S+SS
    // B-curve: SS+S+SS-S-SS
    static ref SYSTEM: LSystem = LSystem::new(
        String::from("A"),
        &[
            ('A', "ASBSA-S-BSASB+S+ASBSA"),
            ('B', "BSASB+S+ASBSA-S-BSASB"),
        ],
    );
}

pub fn model(_app: &App) -> SymbolReader {
    let actions = HashMap::from([
        ('A', Action::None),
        ('B', Action::None),
        ('S', Action::DrawForward(15.0)),
        ('+', Action::RotateRad(1.5708)),
        ('-', Action::RotateRad(-1.5708)),
    ]);

    let cursor = Cursor::new((-194.99623, -194.99829), (0.0, 1.0));

    SymbolReader::new(Box::new(SYSTEM.builder(4)), actions, cursor)
}

pub fn view(app: &App, model: &SymbolReader, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for segment in model.segments.iter() {
        segment
            .line(&draw)
            .rgba(0.776, 0.811, 0.266, 1.0)
            .weight(1.0)
            .caps_round();
    }

    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p INSERT_NAME.mp4

    // use crate::capture::captured_frame_path;
    // let file_path = captured_frame_path(app, &frame, "hilbert_curve");
    // app.main_window().capture_frame(file_path);
}

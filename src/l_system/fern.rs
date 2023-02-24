use std::collections::HashMap;

use lindenmayer::LSystem;
use nannou::{prelude::BLACK, App, Frame};

use super::{cursor::Cursor, Action, SymbolReader};

pub fn model(_app: &App) -> SymbolReader {
    let expression = LSystem::new(
        String::from("X"),
        &[
            ('X', "F+[[X]-X]-F[-FX]+X"),
            ('F', "FF"),
            ('+', "+"),
            ('-', "-"),
            ('[', "["),
            (']', "]"),
        ],
    );

    let actions = HashMap::from([
        ('X', Action::None),
        ('F', Action::DrawForward(25.0)),
        ('+', Action::RotateRad(-0.436332)),
        ('-', Action::RotateRad(0.436332)),
        ('[', Action::PushCursor),
        (']', Action::PopCursor),
    ]);

    let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));

    SymbolReader::new(expression.string(4), actions, cursor)
}

pub fn view(app: &App, model: &SymbolReader, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for segment in model.segments.iter() {
        segment
            .line(&draw)
            .rgba(0.776, 0.811, 0.266, 0.5)
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

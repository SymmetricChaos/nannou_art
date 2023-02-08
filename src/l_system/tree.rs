use std::collections::HashMap;

use nannou::{prelude::BLACK, App, Frame};

use super::{expression::LSystemBuilder, Action, Cursor, LSystem};

pub fn model(_app: &App) -> LSystem {
    let expression = LSystemBuilder::new(
        "X",
        HashMap::from([
            ('X', "F[X][+DX]-DX"),
            ('D', "F"),
            ('F', "F"),
            ('+', "+"),
            ('-', "-"),
            ('[', "["),
            (']', "]"),
        ]),
        7,
    );

    let actions = HashMap::from([
        ('X', Action::None),
        ('D', Action::Dot),
        ('F', Action::DrawForward(40.0)),
        ('+', Action::RotateDeg(-25.0)),
        ('-', Action::RotateDeg(25.0)),
        ('[', Action::PushCursor),
        (']', Action::PopCursor),
    ]);

    let cursor = Cursor::new((0.0, -200.0), (0.0, 1.0));

    LSystem::new(Box::new(expression), actions, cursor)
}

pub fn view(app: &App, model: &LSystem, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for segment in model.segments.iter() {
        segment
            .line(&draw)
            .rgba(0.5, 0.3, 0.9, 0.2)
            .weight(2.0)
            .caps_round();
    }

    for dot in model.dots.iter() {
        draw.ellipse().xy(*dot).radius(3.0).rgba(0.9, 0.2, 0.2, 0.2);
    }

    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p INSERT_NAME.mp4

    // use crate::capture::captured_frame_path;
    // let file_path = captured_frame_path(app, &frame, "hilbert_curve");
    // app.main_window().capture_frame(file_path);
}
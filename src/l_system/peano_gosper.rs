use std::collections::HashMap;

use nannou::{prelude::BLACK, App, Frame};

use super::{expression::LSystemBuilder, Action, Cursor, LSystem};

pub fn model(_app: &App) -> LSystem {
    let expression = LSystemBuilder::new(
        String::from("X"),
        HashMap::from([
            ('X', "X+YF++YF-FX--FXFX-YF+"),
            ('Y', "-FX+YFYF++YF+FX--FX-Y"),
        ]),
        4,
    );

    let actions = HashMap::from([
        ('X', Action::None),
        ('Y', Action::None),
        ('F', Action::DrawForward(10.0)),
        ('+', Action::RotateRad(-1.0472)),
        ('-', Action::RotateRad(1.0472)),
    ]);

    let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));

    LSystem::new(Box::new(expression), actions, cursor)
}

pub fn view(app: &App, model: &LSystem, frame: Frame) {
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

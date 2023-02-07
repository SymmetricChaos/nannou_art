use std::collections::HashMap;

use nannou::{
    prelude::{BLACK, RED},
    App, Frame,
};

use super::{expression::LSystemExpr, Action, Cursor, LSystem};

pub fn model(_app: &App) -> LSystem {
    // A-curve: ss+s+ss-s-ss (NW)
    // B-curve: ss-s-ss+s+ss (NE)

    // C-curve: +ss-s-ss+s+ss- (NW)
    // D-curve: -ss+s+ss-s-ss+ (NE)

    // In the replacement part of the rules A and C can be switched as can B and D
    let expression = LSystemExpr::new(
        String::from("-A"),
        HashMap::from([
            ('A', "AsDsC+s+DsCsD-s-AsBsA"),
            ('B', "DsCsB-s-AsBsA+s+BsAsB"),
            ('C', "+BsAsD-s-AsBsA+s+BsAsB-"),
            ('D', "-AsBsA+s+BsCsB-s-AsBsA+"),
        ]),
        3,
    );

    let actions = HashMap::from([
        ('A', Action::None),
        ('B', Action::None),
        ('C', Action::None),
        ('D', Action::None),
        ('s', Action::DrawForward(15.0)),
        ('+', Action::RotateRad(1.5708)),
        ('-', Action::RotateRad(-1.5708)),
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

    draw.ellipse().radius(5.0).color(RED);

    draw.to_frame(app, &frame).unwrap();

    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p INSERT_NAME.mp4

    // use crate::capture::captured_frame_path;
    // let file_path = captured_frame_path(app, &frame, "hilbert_curve");
    // app.main_window().capture_frame(file_path);
}

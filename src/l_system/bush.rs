// use std::collections::HashMap;

// use lindenmayer::{LSystemBuilderStochastic, LSystemStochastic};
// use nannou::{prelude::BLACK, App, Frame};

// use super::{cursor::Cursor, Action, SymbolReader};

// use lazy_static::lazy_static;

// lazy_static! {
//     static ref SYSTEM: SymbolReader<LSystemBuilderStochastic<'static>> = {
//         let system = LSystemStochastic::new(
//             String::from("X"),
//             &[
//                 ('X', vec![("F[X][+DX]-DX", 1.0)]),
//                 ('F', vec![("L", 1.0), ("S", 1.0)]),
//                 ('L', vec![("L", 1.0)]),
//                 ('S', vec![("S", 1.0)]),
//                 ('D', vec![("D", 1.0)]),
//                 ('+', vec![("+", 1.0)]),
//                 ('-', vec![("-", 1.0)]),
//                 ('[', vec![("[", 1.0)]),
//                 (']', vec![("]", 1.0)]),
//             ],
//         );

//         let actions = HashMap::from([
//             ('F', Action::None),
//             ('X', Action::None),
//             ('L', Action::DrawForward(35.0)),
//             ('S', Action::DrawForward(20.0)),
//             ('D', Action::PushPosition),
//             ('+', Action::RotateRad(-0.4)),
//             ('-', Action::RotateRad(0.4)),
//             ('[', Action::PushCursor),
//             (']', Action::PopCursor),
//         ]);

//         let cursor = Cursor::new((0.0, 0.0), (0.0, 1.0));

//         SymbolReader::new(system.builder(6, None), actions, cursor)
//     };
// }

// pub fn model(_app: &App) -> SymbolReader<LSystemBuilderStochastic> {
//     SYSTEM.clone()
// }

// pub fn view<I: Iterator<Item = char>>(app: &App, model: &SymbolReader<I>, frame: Frame) {
//     let draw = app.draw();

//     draw.background().color(BLACK);

//     for segment in model.segments.iter() {
//         segment
//             .line(&draw)
//             .rgba(0.5, 0.9, 0.266, 0.2)
//             .weight(2.0)
//             .caps_round();
//     }

//     for dot in model.positions.iter() {
//         draw.ellipse()
//             .xy(*dot)
//             .radius(3.0)
//             .rgba(0.5, 0.9, 0.266, 0.2);
//     }

//     draw.to_frame(app, &frame).unwrap();

//     // To create am mp4 from the images use the command below from the directory they are saved to
//     // ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p INSERT_NAME.mp4

//     // use crate::capture::captured_frame_path;
//     // let file_path = captured_frame_path(app, &frame, "hilbert_curve");
//     // app.main_window().capture_frame(file_path);
// }

use itertools_num::linspace;
use nannou::prelude::*;

use crate::helper::captured_frame_path;

pub fn view(app: &App, frame: Frame) {
    if frame.nth() > 188 {
        app.quit()
    }

    let draw = app.draw();

    // Background area
    draw.background().color(PLUM);
    draw.rect().w_h(800.0, 400.0).color(BLACK);

    // Set the animation's time based on the frame
    let t = (frame.nth() as f32) / 30.0;

    // Set the overall scale ofthe circle and draw it.
    let scale = 50.0;
    draw.ellipse().w_h(scale * 2.0, scale * 2.0).color(GRAY);

    // Create the dot that will move around the circle and the bar connected to it
    let x = t.cos() * scale;
    let y = t.sin() * scale;
    let moving_dot = pt2(x, y);

    draw.line()
        .start(moving_dot)
        .end((150.0, y).into())
        .weight(3.0)
        .color(SALMON);
    draw.ellipse().w_h(10.0, 10.0).x_y(x, y);

    let wave = linspace::<f32>(150.0, 400.0, 200).map(|n| {
        let pt = pt2(n, ((0.1 * (n - 150.0)) + t).sin() * scale);
        (pt, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(wave);

    draw.ellipse().w_h(10.0, 10.0).x_y(150.0, y);

    draw.to_frame(app, &frame).unwrap();
    app.main_window()
        .capture_frame(captured_frame_path(&app, &frame, "wave"));
}

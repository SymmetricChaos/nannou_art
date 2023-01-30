use nannou::prelude::*;

pub fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.rect().w_h(800.0, 400.0).color(BLACK);

    let t = frame.nth() as f32 / 30.0;
    draw.quad().color(DARKGREEN).rotate(t);

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    // To create am mp4 from the images use the command below from the directory they are saved to
    // ffmpeg -r 60 -f image2 -s 1920x1080 -i %03d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p test.mp4
    let file_path = captured_frame_path(app, &frame, "rotating_square");
    app.main_window().capture_frame(file_path);
}

pub fn captured_frame_path(app: &App, frame: &Frame, dir_name: &str) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(dir_name)
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}

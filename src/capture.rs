use nannou::prelude::*;

pub fn captured_frame_path(app: &App, frame: &Frame, dir_name: &str) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(dir_name)
        // Name each file after the number of the frame.
        .join(format!("{:04}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}

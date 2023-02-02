use nannou::prelude::*;

// To create am mp4 from the images use the command below from the directory they are saved to
// ffmpeg -r 30 -f image2 -s 1920x1080 -i %04d.png -vcodec libx264 -crf 25  -pix_fmt yuv420p INSERT_NAME.mp4
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

use nannou::{
    draw::{primitive::Line, Drawing},
    prelude::Vec2,
    App, Draw, Frame,
};

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

pub struct Segment {
    start: Vec2,
    end: Vec2,
}

impl Segment {
    pub fn scaled(mut self, scale: f32) -> Self {
        self.start *= scale;
        self.end *= scale;
        self
    }

    pub fn line<'a>(&'a self, draw: &'a Draw) -> Drawing<Line> {
        draw.line().start(self.start).end(self.end)
    }
}

impl From<((i32, i32), (i32, i32))> for Segment {
    fn from(value: ((i32, i32), (i32, i32))) -> Self {
        let start = (value.0 .0 as f32, value.0 .1 as f32).into();
        let end = (value.1 .0 as f32, value.1 .1 as f32).into();
        Self { start, end }
    }
}

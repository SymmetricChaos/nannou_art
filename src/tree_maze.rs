use nannou::prelude::*;

pub struct Segment {
    start: Vec2,
    end: Vec2,
}

const NUM_ROOMS: f32 = 25.0;
pub struct Model {
    segments: Vec<Segment>,
    position: Vec2,
    room_width: f32,
    x_lims: (f32, f32),
    y_lims: (f32, f32),
}

pub fn model(app: &App) -> Model {
    let rect = app.window_rect().pad(50.0);
    Model {
        segments: Vec::new(),
        position: rect.bottom_left(),
        room_width: rect.w() / NUM_ROOMS,
        x_lims: (rect.left(), rect.right()),
        y_lims: (rect.bottom(), rect.top()),
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.position.y >= model.y_lims.1 - model.room_width {
        return;
    }
    while model.position.x <= model.x_lims.1 - model.room_width {
        let line = match nannou::rand::random::<bool>() {
            true => Segment {
                start: model.position,
                end: model.position + pt2(0.0, model.room_width),
            },
            false => Segment {
                start: model.position,
                end: model.position + pt2(model.room_width, 0.0),
            },
        };
        model.segments.push(line);
        model.position += pt2(model.room_width, 0.0);
    }
    model.position.y += model.room_width;
    model.position.x = model.x_lims.0;
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(STEELBLUE);
    let rect = app.window_rect().pad(50.0);
    draw.rect()
        .no_fill()
        .w_h(rect.w(), rect.h())
        .stroke_color(BLACK)
        .stroke_weight(3.0);
    for segment in model.segments.iter() {
        draw.line()
            .start(segment.start)
            .end(segment.end)
            .color(BLACK)
            .weight(2.0);
    }
    draw.to_frame(app, &frame).unwrap();
}

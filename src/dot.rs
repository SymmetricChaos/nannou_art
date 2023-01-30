use nannou::prelude::*;

pub struct Model {
    x: f32,
    y: f32,
    radius: f32,
    radius_step: f32,
}

pub fn model(_app: &App) -> Model {
    Model {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
        radius_step: 1.0,
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.radius >= 300.0 || model.radius <= 10.0 {
        model.radius_step *= -1.0;
    }

    model.radius += model.radius_step;
    model.radius = model.radius.clamp(10.0, 300.0);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(HONEYDEW);
    draw.ellipse()
        .color(STEELBLUE)
        .w(model.radius)
        .h(model.radius)
        .x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
    
}

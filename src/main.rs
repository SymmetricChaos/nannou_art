pub mod capture;
pub mod dot;
pub mod helper;
pub mod maze;
pub mod space_filling;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(space_filling::hilbert::model)
        .update(space_filling::hilbert::update)
        .simple_window(space_filling::hilbert::view)
        .run();
}

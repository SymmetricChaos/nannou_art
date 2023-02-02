pub mod capture;
pub mod dot;
pub mod maze;
pub mod segment;
pub mod space_filling;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(space_filling::l_system::model)
        .update(space_filling::l_system::update)
        .simple_window(space_filling::l_system::view)
        .run();
}

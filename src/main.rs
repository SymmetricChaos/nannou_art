pub mod capture;
pub mod dot;
pub mod l_system;
pub mod maze;
pub mod segment;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(l_system::corn::model)
        .update(l_system::update_fast)
        .simple_window(l_system::corn::view)
        .run();
}

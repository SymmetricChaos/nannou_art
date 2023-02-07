pub mod capture;
pub mod dot;
pub mod graph;
pub mod l_system;
pub mod maze;
pub mod segment;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(l_system::tree::model)
        .update(l_system::draw)
        .simple_window(l_system::tree::view)
        .run();
}

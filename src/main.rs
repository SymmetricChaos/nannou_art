pub mod capture;
pub mod dot;
pub mod l_system;
pub mod maze;
pub mod segment;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(l_system::bush::model)
        .update(l_system::draw)
        .simple_window(l_system::bush::view)
        .run();
}

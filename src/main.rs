pub mod breadth_first_maze;
pub mod capture;
pub mod depth_first_maze;
pub mod dot;
pub mod helper;
pub mod some_maze;
pub mod tree_maze;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(breadth_first_maze::model)
        .update(breadth_first_maze::update)
        .simple_window(breadth_first_maze::view)
        .run();
}

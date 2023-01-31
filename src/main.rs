pub mod capture;
pub mod dot;
pub mod helper;
pub mod maze;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(maze::depth_first_maze::model)
        .update(maze::depth_first_maze::update)
        .simple_window(maze::depth_first_maze::view)
        .run();
}

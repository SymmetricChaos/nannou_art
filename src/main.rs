pub mod capture;
pub mod dot;
pub mod helper;
pub mod maze;
pub mod space_filling;
pub mod wave;

fn main() {
    //nannou::sketch(wave::view).run();
    nannou::app(maze::breadth_first_maze_growing::model)
        .update(maze::breadth_first_maze_growing::update)
        .simple_window(maze::breadth_first_maze_growing::view)
        .run();
}

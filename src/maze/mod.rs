use itertools::Itertools;
use nannou::{prelude::BLACK, Draw};

pub mod breadth_first_maze;
pub mod depth_first_maze;

pub trait SquareMaze {
    fn cursor(&self) -> (i32, i32);
    fn scale(&self) -> f32;
    fn width(&self) -> usize;
    fn cells(&self) -> &mut Vec<(i32, i32)>;

    fn neighbors(&self, p: (i32, i32)) -> Vec<(i32, i32)> {
        [
            (p.0 - 1, p.1),
            (p.0 + 1, p.1),
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
        ]
        .iter()
        .filter(|x| self.cells().contains(x))
        .copied()
        .collect_vec()
    }

    fn outline(&self, draw: &Draw) {
        let r_size = (self.width() + 1) as f32 * self.scale() * 2.0;
        draw.rect()
            .no_fill()
            .w_h(r_size, r_size)
            .stroke_color(BLACK)
            .stroke_weight(3.0);
    }
}

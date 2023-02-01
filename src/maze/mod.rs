use itertools::Itertools;
use nannou::Draw;

pub mod breadth_first_maze;
pub mod depth_first_maze;

pub trait SquareMaze {
    fn cursor(&self) -> (i32, i32);
    fn scale(&self) -> f32;
    fn width(&self) -> usize;
    fn cells(&self) -> &mut Vec<(i32, i32)>;
    fn neighbors(&self, p: (i32, i32)) -> Vec<(i32, i32)>;

    fn outline<'a>(
        &'a self,
        draw: &'a Draw,
    ) -> nannou::draw::Drawing<nannou::draw::primitive::Rect> {
        let r_size = (self.width() + 1) as f32 * self.scale() * 2.0;
        draw.rect().w_h(r_size, r_size)
    }
}

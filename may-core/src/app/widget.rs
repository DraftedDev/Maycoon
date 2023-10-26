use taffy::layout::Layout;
use taffy::style::Style;

use crate::canvas::Canvas;

pub trait Widget {
    fn draw(&mut self, canvas: &mut Canvas, layout: &Layout);
    fn get_style(&self) -> Style;
    fn get_children(&self) -> Vec<Box<dyn Widget>>;
}

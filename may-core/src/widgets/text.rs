use skia_safe::{Color, Color4f, ColorSpace, Font, Paint, Point};
use taffy::layout::Layout;
use taffy::prelude::Size;
use taffy::style::{Dimension, Style};

use crate::app::widget::Widget;
use crate::canvas::Canvas;

pub struct Text {
    text: String,
    paint: Paint,
    style: Style,
    font: Font,
}

impl Text {
    pub fn new(text: impl ToString) -> Box<Self> {
        let mut paint = Paint::new(Color4f::from(Color::BLACK), &ColorSpace::new_srgb());
        paint.set_anti_alias(true);

        let font = Font::default();

        Box::new(Self {
            text: text.to_string(),
            paint,
            style: Style {
                size: Size::<Dimension>::from_points(15.0, 15.0),
                ..Default::default()
            },
            font,
        })
    }

    pub fn append(mut self, text: impl ToString) -> Self {
        self.text.push_str(text.to_string().as_str());
        self
    }

    pub fn with_text(mut self, text: impl ToString) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_paint(mut self, paint: Paint) -> Self {
        self.paint = paint;
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn mut_style(mut self, mutation: impl FnOnce(&mut Style)) -> Self {
        mutation(&mut self.style);
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.style.size = Size::from_points(size, size);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.paint.set_color(color);
        self
    }
}

impl Widget for Text {
    fn draw(&mut self, canvas: &mut Canvas, layout: &Layout) {
        let pos = layout.location;
        let size = layout.size.width;

        self.font.set_size(size);

        canvas.skia_canvas().draw_str(
            &self.text,
            Point::new(pos.x + size, pos.y + size),
            &self.font,
            &self.paint,
        );
    }

    fn get_style(&self) -> Style {
        self.style.clone()
    }

    fn get_children(&self) -> Vec<Box<dyn Widget>> {
        vec![]
    }
}

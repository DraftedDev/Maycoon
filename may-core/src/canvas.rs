use skia_safe::{surfaces, ISize, Paint, Path, Surface};
use taffy::geometry::Size;
use taffy::node::Node;
use taffy::style::{AvailableSpace, Dimension, Style};
use taffy::Taffy;

use crate::types::WidgetTree;

pub struct Canvas {
    surface: Surface,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        let surface = surfaces::raster_n32_premul(ISize::new(width, height))
            .expect("Skia: Failed to create surface");
        Self { surface }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.surface = surfaces::raster_n32_premul(ISize::new(width, height))
            .expect("Skia: Failed to create surface when resizing Canvas");
    }

    pub fn data(&mut self) -> Vec<u32> {
        let pixels: &[u32] = self
            .surface
            .peek_pixels()
            .expect("Skia: Failed getting PixelMap")
            .pixels()
            .expect("Skia: Failed getting pixels");

        pixels.to_vec()
    }

    pub fn draw(&mut self, path: &Path, paint: &Paint) {
        self.surface.canvas().draw_path(path, paint);
    }

    pub fn draw_root(&mut self, widgets: &mut WidgetTree, width: f32, height: f32) {
        let mut taffy = Taffy::new();

        let root = taffy
            .new_leaf(Style {
                size: Size::<Dimension>::from_percent(width, height),
                ..Default::default()
            })
            .expect("Taffy: Failed to create root leaf");

        self.draw_widgets(widgets, &mut taffy, root);
    }

    pub fn draw_widgets(&mut self, widgets: &mut WidgetTree, taffy: &mut Taffy, root: Node) {
        widgets.iter_mut().for_each(|w| {
            let child = taffy
                .new_leaf(w.get_style())
                .expect("Taffy: Failed to create widget node");
            taffy
                .add_child(root, child)
                .expect("Taffy: Failed to add child");

            taffy
                .compute_layout(root, Size::<AvailableSpace>::max_content())
                .expect("Taffy: Failed to compute layout");

            self.draw_widgets(&mut w.get_children(), taffy, Default::default());
            w.draw(self, taffy.layout(child).unwrap());
        });
    }

    pub fn skia_canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }
}

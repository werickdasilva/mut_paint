use crate::geometry::Point;
use gtk::{
    cairo::{Context, ImageSurface},
};

pub struct Canvas {
    image: Option<ImageSurface>,
    pub position: Point,
    pub zoom: f64
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            image: None,
            position: Point::ZERO,
            zoom: 1.0,
        }
    }

    pub fn open(&mut self, image: ImageSurface) {
        self.image = Some(image);
    }

    pub fn draw(&self, ctx: &Context) {
        if let Some(image) = self.image.as_ref() {
            ctx.save().unwrap();
            ctx.translate(self.position.x, self.position.y);
            ctx.scale(self.zoom, self.zoom);
            ctx.set_source_surface(image, 0., 0.).unwrap();
            ctx.paint().unwrap();
            ctx.restore().unwrap();
        }
    }
}

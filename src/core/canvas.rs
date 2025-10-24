use crate::geometry::Point;
use gtk::cairo::{Context, ImageSurface, Matrix};

pub struct Canvas {
    image: Option<ImageSurface>,
    pub position: Point,
    pub zoom: f64,
    pub rotation: f64,
    matrix: Matrix,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            image: None,
            position: Point::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            matrix: Matrix::identity(),
        }
    }

    pub fn open(&mut self, image: ImageSurface) {
        self.image = Some(image);
    }

    pub fn get_image_context(&self) -> Option<Context> {
        self.image
            .as_ref()
            .and_then(|img| Context::new(img.clone()).ok())
    }

    pub fn screen_to_canvas_coords(&self, screen_point: Point) -> Point {
        let mut matrix = self.matrix;
        matrix.invert();
        let (x, y) = matrix.transform_point(screen_point.x, screen_point.y);
        Point { x, y }
    }

    pub fn draw(&mut self, ctx: &Context) {
        if let Some(image) = self.image.as_ref() {
            let half_width = image.width() as f64 / 2.;
            let half_height = image.height() as f64 / 2.;
            ctx.save().unwrap();

            ctx.translate(self.position.x, self.position.y);
            ctx.rotate(self.rotation);
            ctx.scale(self.zoom, self.zoom);
            ctx.translate(-half_width, -half_height);

            self.matrix = ctx.matrix();

            ctx.set_source_surface(image, 0., 0.).unwrap();
            ctx.paint().unwrap();
            ctx.restore().unwrap();
        }
    }
}

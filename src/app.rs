use gtk::{cairo::Context, gdk::prelude::GdkCairoContextExt, gdk_pixbuf::Pixbuf};

pub struct App {
    image: Option<Pixbuf>,
}

impl App {
    pub fn new() -> Self {
        App { image: None }
    }

    pub fn open(&mut self, image: Pixbuf) {
        self.image = Some(image)
    }

    pub fn draw(&self, ctx: &Context) {
        if let Some(image) = self.image.as_ref() {
            ctx.set_source_pixbuf(image, 0., 0.);
            ctx.paint().unwrap()
        }
    }
}

use crate::app::App;
use gtk::{cairo::Context, gdk_pixbuf::Pixbuf};
use std::cell::RefCell;

pub struct Program {
    app: RefCell<App>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            app: RefCell::new(App::new()),
        }
    }

    pub fn open_image(&self, path: impl Into<String>) {
        let pixbuf = Pixbuf::from_file(path.into());
        if let Ok(image) = pixbuf {
            self.app.borrow_mut().open(image);
        }
    }

    pub fn draw(&self, ctx: &Context) {
        self.app.borrow().draw(ctx);
    }
}

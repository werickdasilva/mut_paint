use crate::core::{app::App, event::AppEvents};
use gtk::{cairo::Context, gdk_pixbuf::Pixbuf};
use std::cell::RefCell;

#[derive(Default)]
pub struct ProgramState {
    needs_paint: bool,
}

impl ProgramState {
    pub fn request_paint(&mut self) {
        self.needs_paint = true;
    }

    pub fn stop_request_paint(&mut self) {
        self.needs_paint = false;
    }

    pub fn needs_paint(&self) -> bool {
        self.needs_paint
    }
}

pub struct Program {
    app: RefCell<App>,
    pub state: RefCell<ProgramState>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            app: RefCell::new(App::new()),
            state: RefCell::default(),
        }
    }

    pub fn open_image(&self, path: impl Into<String>) {
        let pixbuf = Pixbuf::from_file(path.into());
        if let Ok(image) = pixbuf {
            self.app.borrow_mut().open(image);
        }
    }

    pub fn on_event(&self, events: AppEvents) {
        self.app
            .borrow_mut()
            .on_event(events, &mut self.state.borrow_mut());
    }

    pub fn draw(&self, ctx: &Context) {
        self.app.borrow().draw(ctx);
    }
}

use crate::{canvas::Canvas, event::AppEvents, program::ProgramState, tools::PanTool};
use gtk::{
    cairo::{Context, Format, ImageSurface},
    gdk::prelude::GdkCairoContextExt,
    gdk_pixbuf::Pixbuf,
};

pub struct App {
    canvas: Canvas,
    pan: PanTool,
}

impl App {
    pub fn new() -> Self {
        App {
            canvas: Canvas::new(),
            pan: PanTool::new(),
        }
    }

    pub fn open(&mut self, pixbuf: Pixbuf) {
        let image = ImageSurface::create(Format::ARgb32, pixbuf.width(), pixbuf.height()).unwrap();
        let ctx = Context::new(&image).unwrap();
        ctx.set_source_pixbuf(&pixbuf, 0., 0.);
        ctx.paint().unwrap();

        self.canvas.open(image);
    }

    pub fn on_event(&mut self, events: AppEvents, state: &mut ProgramState) {
        self.pan.on_event(events, &mut self.canvas, state);
    }

    pub fn draw(&self, ctx: &Context) {
        self.canvas.draw(ctx);
    }
}

use crate::{
    core::{canvas::Canvas, event::AppEvents},
    program::ProgramState,
    tools::{PanTool, ZoomTool},
};
use gtk::{
    cairo::{Context, Format, ImageSurface},
    gdk::prelude::GdkCairoContextExt,
    gdk_pixbuf::Pixbuf,
};
pub struct App {
    canvas: Canvas,
    pan: PanTool,
    zoom: ZoomTool,
}

impl App {
    pub fn new() -> Self {
        App {
            canvas: Canvas::new(),
            pan: PanTool::new(),
            zoom: ZoomTool::new(),
        }
    }
    pub fn zoom_in(&mut self) {
        self.zoom.zoom_in(&mut self.canvas);
    }
    pub fn zoom_out(&mut self) {
        self.zoom.zoom_out(&mut self.canvas);
    }

    pub fn get_zoom(&self) -> f64 {
        self.canvas.zoom * 100.
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
        self.zoom.on_event(events, &mut self.canvas, state);
    }

    pub fn draw(&self, ctx: &Context) {
        self.canvas.draw(ctx);
    }
}

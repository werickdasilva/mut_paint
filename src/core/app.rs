use std::{fmt, str::FromStr};

use crate::{
    core::{canvas::Canvas, event::AppEvents},
    program::ProgramState,
    tools::{BrushTool, PanTool, RotateTool, ZoomTool},
};
use gtk::{
    cairo::{Context, Format, ImageSurface},
    gdk::prelude::GdkCairoContextExt,
    gdk_pixbuf::Pixbuf, glib::variant::ToVariant,
};

pub enum Tools {
    Pan,
    Brush,
}

impl Tools {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tools::Pan => "pan",
            Tools::Brush => "brush"
        }
    }
}

impl fmt::Display for Tools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Tools {
    type Err  = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pan" => Ok(Tools::Pan),
            "brush" => Ok(Tools::Brush),
            _ => Err("Tool Invalid")
        }
    }
}


pub struct App {
    canvas: Canvas,
    pan: PanTool,
    zoom: ZoomTool,
    rotate: RotateTool,
    brush: BrushTool,
    active_tool: Tools
}

impl App {
    pub fn new() -> Self {
        App {
            canvas: Canvas::new(),
            pan: PanTool::new(),
            zoom: ZoomTool::new(),
            rotate: RotateTool::new(),
            brush: BrushTool::new(),
            active_tool: Tools::Pan,
        }
    }

    pub fn set_tool(&mut self, tool: Tools) {
        self.active_tool = tool;
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
    pub fn get_rotate(&self) -> f64 {
        self.canvas.rotation.to_degrees()
    }

    pub fn rotate_left(&mut self) {
        self.rotate.rotate_left(&mut self.canvas);
    }

    pub fn rotate_right(&mut self) {
        self.rotate.rotate_right(&mut self.canvas);
    }

    pub fn open(&mut self, pixbuf: Pixbuf) {
        let image = ImageSurface::create(Format::ARgb32, pixbuf.width(), pixbuf.height()).unwrap();
        let ctx = Context::new(&image).unwrap();
        ctx.set_source_pixbuf(&pixbuf, 0., 0.);
        ctx.paint().unwrap();

        self.canvas.open(image);
    }

    pub fn on_event(&mut self, events: AppEvents, state: &mut ProgramState) {
        match self.active_tool {
            Tools::Pan => self.pan.on_event(events, &mut self.canvas, state),
            Tools::Brush => self.brush.on_event(events, &mut self.canvas, state),
        }
        self.zoom.on_event(events, &mut self.canvas, state);
    }

    pub fn draw(&mut self, ctx: &Context) {
        self.canvas.draw(ctx);
    }
}

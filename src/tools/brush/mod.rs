pub(crate) mod brush_definition;
mod brush_manager;

use crate::{
    core::{canvas::Canvas, event::AppEvents},
    geometry::Point,
    program::ProgramState,
};
use brush_manager::BrushManager;
use gtk::cairo::Context;
use rand::Rng;

pub struct BrushTool {
    brush_manager: BrushManager,
    is_drawing: bool,
    last_point: Point,
    thickness: f64,
    spacing: f64,
}

impl BrushTool {
    pub fn new() -> Self {
        let mut brush_manager = BrushManager::new();
        brush_manager.load_default().unwrap();

        BrushTool {
            brush_manager: brush_manager,
            is_drawing: false,
            last_point: Point::ZERO,
            thickness: 15.0,
            spacing: 0.1,
        }
    }

    pub fn on_event(&mut self, events: AppEvents, canvas: &mut Canvas, state: &mut ProgramState) {
        match events {
            AppEvents::MouseDown(point) => {
                if let Some(ctx) = canvas.get_image_context() {
                    let image_point = canvas.screen_to_canvas_coords(point);
                    self.last_point = image_point;
                    self.is_drawing = true;
                    self.draw_stamp(&ctx, image_point);
                    state.request_paint();
                }
            }
            AppEvents::MouseMove(point) => {
                if !self.is_drawing {
                    return;
                }

                if let Some(ctx) = canvas.get_image_context() {
                    let image_point = canvas.screen_to_canvas_coords(point);
                    self.draw_stamps(&ctx, self.last_point, image_point);
                    self.last_point = image_point;
                    state.request_paint();
                }
            }
            AppEvents::MouseUp(_) => {
                self.is_drawing = false;
                state.stop_request_paint();
            }
            _ => {}
        }
    }

    fn draw_stamp(&mut self, ctx: &Context, image_point: Point) {
        let mut rng = rand::rng();
        let angle_step = 15;
        let n_steps = 360 / angle_step;
        let angle_index = rng.random_range(0..n_steps);
        let angle_deg = angle_index * angle_step;

        if let Some(brush) = self.brush_manager.get_active_brush_mut() {
            let cached = brush.get_cached(self.thickness, angle_deg);

            ctx.save().unwrap();
            let x = image_point.x - (cached.width() as f64) / 2.0;
            let y = image_point.y - (cached.height() as f64) / 2.0;
            ctx.set_source_surface(cached, x, y).unwrap();
            ctx.paint().unwrap();
            ctx.restore().unwrap();
        }
    }

    fn draw_stamps(&mut self, ctx: &Context, start: Point, end: Point) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let distance = dx.hypot(dy);

        const CONTINUOUS_SPACING_FACTOR: f64 = 0.7;
        let ideal_continuous_spacing = self.thickness * CONTINUOUS_SPACING_FACTOR;
        let spacing_use = self.spacing.min(ideal_continuous_spacing).max(1.0);
        let steps = (distance / spacing_use).ceil() as u32;

        if steps == 0 {
            self.draw_stamp(ctx, end);
            return;
        }

        for i in 0..=steps {
            let t = (i as f64) / (steps as f64);
            let p = Point {
                x: start.x + dx * t,
                y: start.y + dy * t,
            };
            self.draw_stamp(ctx, p);
        }
    }
}

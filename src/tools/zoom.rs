use crate::canvas::Canvas;
use crate::event::AppEvents;
use crate::geometry::Point;
use crate::program::ProgramState;

pub struct ZoomTool {
    point: Point,
}

impl ZoomTool {
    pub fn new() -> Self {
        ZoomTool { point: Point::ZERO }
    }
    pub fn on_event(&mut self, events: AppEvents, canvas: &mut Canvas, state: &mut ProgramState) {
        match events {
            AppEvents::MouseMove(point) => self.point = point,
            AppEvents::ScroolEvent { delta_y } => {
                let zoom_factor = if delta_y < 0.0 { 1.1 } else { 1.0 / 1.1 };
                let pre_zoom_x = (self.point.x - canvas.position.x) /canvas.zoom;
                let pre_zoom_y = (self.point.y - canvas.position.y) /canvas.zoom;

                let new_zoom = (canvas.zoom * zoom_factor).max(0.1).min(10.0);
                if new_zoom != canvas.zoom {
                    canvas.zoom = new_zoom;
                    canvas.position.x = self.point.x - (pre_zoom_x * new_zoom);
                    canvas.position.y = self.point.y - (pre_zoom_y * new_zoom);
                    state.request_paint();
                }

            }
            _ => {}
        }
    }
}

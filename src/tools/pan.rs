use crate::canvas::Canvas;
use crate::event::AppEvents;
use crate::geometry::Point;
use crate::program::ProgramState;

pub struct PanTool {
    last_point: Option<Point>,
}

impl PanTool {
    pub fn new() -> Self {
        PanTool { last_point: None }
    }
    pub fn on_event(&mut self, events: AppEvents, canvas: &mut Canvas, state: &mut ProgramState) {
        match events {
            AppEvents::MouseDown(point) => {
                self.last_point = Some(point);
                state.request_paint();
            }
            AppEvents::MouseMove(point) => {
                if let Some(last) = self.last_point {
                    let dx = point.x - last.x;
                    let dy = point.y - last.y;
                    canvas.position.x += dx;
                    canvas.position.y += dy;
                    self.last_point = Some(point);
                    state.request_paint();
                }
            }
            AppEvents::MouseUp(_) => {
                self.last_point = None;
                state.stop_request_paint();
            }
        }
    }
}

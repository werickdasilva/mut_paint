use crate::geometry::Point;

#[derive(Debug, Clone, Copy)]
pub enum AppEvents {
    MouseUp(Point),
    MouseMove(Point),
    MouseDown(Point),
    ScroolEvent{delta_y: f64}
}
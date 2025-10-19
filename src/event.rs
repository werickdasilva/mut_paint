use crate::geometry::Point;

pub enum AppEvents {
    MouseUp(Point),
    MouseMove(Point),
    MouseDown(Point),
}
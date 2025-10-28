pub const OPEN_IMAGE: &str = "open-image";
pub const ZOOM_IN: &str = "zoom-in";
pub const ZOOM_OUT: &str = "zoom-out";
pub const EXIT: &str = "exit";
pub const ROTATE_LEFT: &str = "rotate-left";
pub const ROTATE_RIGHT: &str = "rotate-right";
pub const TOOL_PAN: &str = "tool-pan";
pub const TOOL_BRUSH: &str = "tool-brush";

pub mod app {
    use const_format::concatcp;

    const APP_PREFIX: &str = "app.";
    pub const EXIT: &str = concatcp!(APP_PREFIX, super::EXIT);
    pub const OPEN_IMAGE: &str = concatcp!(APP_PREFIX, super::OPEN_IMAGE);
    pub const ZOOM_IN: &str = concatcp!(APP_PREFIX, super::ZOOM_IN);
    pub const ZOOM_OUT: &str = concatcp!(APP_PREFIX, super::ZOOM_OUT);
    pub const ROTATE_LEFT: &str = concatcp!(APP_PREFIX, super::ROTATE_LEFT);
    pub const ROTATE_RIGHT: &str = concatcp!(APP_PREFIX, super::ROTATE_RIGHT);
    pub const TOOL_PAN: &str = concatcp!(APP_PREFIX, super::TOOL_PAN);
    pub const TOOL_BRUSH: &str = concatcp!(APP_PREFIX, super::TOOL_BRUSH);
}

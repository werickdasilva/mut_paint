
pub const OPEN_IMAGE: &str = "open-image";
pub const EXIT: &str = "exit";

pub mod app {
    use const_format::concatcp;

    const APP_PREFIX: &str = "app.";
    pub const EXIT: &str = concatcp!(APP_PREFIX, super::EXIT);
    pub const OPEN_IMAGE: &str = concatcp!(APP_PREFIX, super::OPEN_IMAGE);
}

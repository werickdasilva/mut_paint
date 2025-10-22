use std::path::PathBuf;

use gtk::{
    ApplicationWindow, FileDialog, FileFilter,
    gio::{Cancellable, prelude::FileExt},
};

pub struct OpenImage {
    dialog: FileDialog,
}

impl OpenImage {
    pub fn new() -> Self {
        let filters = Self::default_filters();

        let dialog = FileDialog::builder()
            .default_filter(&filters)
            .title("Open Image")
            .modal(true)
            .build();

        OpenImage { dialog }
    }

    pub fn run<F: Fn(Option<PathBuf>) + 'static>(&self, window: &ApplicationWindow, f: F) {
        self.dialog
            .open(Some(window), None::<&Cancellable>, move |result| {
                if let Ok(file) = result {
                    f(file.path());
                }
            });
    }

    fn default_filters() -> FileFilter {
        let filters = FileFilter::new();
        filters.add_pixbuf_formats();
        filters
    }
}

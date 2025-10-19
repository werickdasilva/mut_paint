
use gtk::{
    Application,
    gio::{
        prelude::{ ApplicationExt, ApplicationExtManual},
    },
};
use mut_paint::ui;

const APP_ID: &str = "mut_paint.MutPaint";

fn main() -> gtk::glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui);
    app.run()
}

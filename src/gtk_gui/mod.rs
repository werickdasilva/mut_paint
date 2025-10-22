pub mod actions;
mod main_window;
mod menu_bar;
mod open_image;

use crate::{
    gtk_gui::{main_window::MainWindow, menu_bar::MenuBar},
    program::Program,
};
use gtk::{
    Application,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::clone,
};
use std::rc::Rc;

const APP_ID: &str = "mut_paint.MutPaint";

pub struct GtkGui {
    main_window: Rc<MainWindow>,
}

impl GtkGui {
    pub fn start(program: Rc<Program>) {
        let application = Application::builder().application_id(APP_ID).build();
        application.connect_startup(|app| {
            MenuBar::new(app);
        });
        application.connect_activate(clone!(
            #[strong]
            program,
            move |gtk_app| {
                MenuBar::new(gtk_app);
                let gui = GtkGui {
                    main_window: Rc::new(MainWindow::new(&gtk_app, Rc::clone(&program))),
                };

                gui.build_ui();
            }
        ));
        application.run();
    }

    fn build_ui(&self) {
        self.main_window.start();
    }
}

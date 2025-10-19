pub mod app;
mod program;
mod ui;

use crate::{program::Program, ui::create_menu_bar};
use gtk::{
    Application, ApplicationWindow, DrawingArea, FileDialog, FileFilter, Orientation,
    gio::{
        self, ActionEntry,
        prelude::{ActionMapExtManual, FileExt},
    },
    glib::clone,
    prelude::{BoxExt, DrawingAreaExtManual, GtkWindowExt, WidgetExt},
};
use std::rc::Rc;

pub fn ui(app: &Application) {
    create_menu_bar(app);

    let program = Rc::new(Program::new());
    let drawing = Rc::new(DrawingArea::new());

    drawing.set_draw_func(clone!(
        #[strong]
        program,
        move |_, ctx, _, _| {
            program.draw(ctx);
        }
    ));
    drawing.set_hexpand(true);
    drawing.set_vexpand(true);

    let v_area = gtk::Box::new(Orientation::Horizontal, 0);
    v_area.append(drawing.as_ref());

    let window = ApplicationWindow::builder()
        .application(app)
        .title("MutPaint")
        .can_focus(true)
        .default_width(700)
        .default_height(500)
        .child(&v_area)
        .show_menubar(true)
        .build();

    let open_file = ActionEntry::builder("open-file")
        .activate(clone!(
            #[strong]
            program,
            #[strong]
            drawing,
            move |window, _, _| {
                let filters = FileFilter::new();
                filters.add_pixbuf_formats();

                let dialog = FileDialog::builder()
                    .default_filter(&filters)
                    .title("Abrir Imagem")
                    .modal(true)
                    .build();
                dialog.open(
                    Some(window),
                    None::<&gio::Cancellable>,
                    clone!(
                        #[strong]
                        program,
                        #[strong]
                        drawing,
                        move |result| {
                            if let Ok(file) = result {
                                let path = file.path().unwrap().to_str().unwrap().to_string();
                                program.open_image(path);
                                drawing.queue_draw();
                            }
                        }
                    ),
                );
            }
        ))
        .build();
    window.add_action_entries([open_file]);
    window.present();
}

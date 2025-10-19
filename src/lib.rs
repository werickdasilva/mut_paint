pub mod app;
mod canvas;
mod event;
mod geometry;
mod program;
mod tools;
mod ui;

use crate::{event::AppEvents, geometry::Point, program::Program, ui::create_menu_bar};
use gtk::{
    Application, ApplicationWindow, DrawingArea, EventControllerMotion, FileDialog, FileFilter,
    GestureClick, Orientation,
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
    set_events_drawing_area(Rc::clone(&drawing), Rc::clone(&program));

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

    let window = ApplicationWindow::builder()
        .application(app)
        .title("MutPaint")
        .can_focus(true)
        .default_width(700)
        .default_height(500)
        .child(&v_area)
        .show_menubar(true)
        .build();
    window.add_action_entries([open_file]);
    window.present();
}

pub fn set_events_drawing_area(drawing: Rc<DrawingArea>, program: Rc<Program>) {
    let motion = EventControllerMotion::new();
    let gesture = GestureClick::new();

    motion.connect_motion(clone!(
        #[strong]
        program,
        #[strong]
        drawing,
        move |_, x, y| {
            program.on_event(AppEvents::MouseMove(Point::new(x, y)));
            if program.state.borrow().needs_paint() {
                drawing.queue_draw();
                program.state.borrow_mut().stop_request_paint();
            }
        }
    ));

    gesture.connect_pressed(clone!(
        #[strong]
        program,
        #[strong]
        drawing,
        move |_, _, x, y| {
            program.on_event(AppEvents::MouseDown(Point::new(x, y)));
            if program.state.borrow().needs_paint() {
                drawing.queue_draw();
                program.state.borrow_mut().stop_request_paint();
            }
        }
    ));
    gesture.connect_released(clone!(
        #[strong]
        program,
        #[strong]
        drawing,
        move |_, _, x, y| {
            program.on_event(AppEvents::MouseUp(Point::new(x, y)));
            if program.state.borrow().needs_paint() {
                drawing.queue_draw();
                program.state.borrow_mut().stop_request_paint();
            }
        }
    ));

    drawing.add_controller(motion);
    drawing.add_controller(gesture);
}

use crate::{
    core::event::AppEvents,
    geometry::Point,
    gtk_gui::{actions, open_image::OpenImage},
    program::Program,
};
use gtk::{
    Application, ApplicationWindow, DrawingArea, EventControllerMotion, EventControllerScroll,
    EventControllerScrollFlags, GestureClick,
    gio::{
        SimpleAction,
        prelude::{ActionMapExt, ApplicationExt},
    },
    glib::{self, clone},
    prelude::{DrawingAreaExtManual, GtkApplicationExt, GtkWindowExt, WidgetExt},
};
use std::rc::Rc;

pub struct MainWindow {
    gtk_app: Application,
    window: ApplicationWindow,
    drawing: Rc<DrawingArea>,
    program: Rc<Program>,
}

impl MainWindow {
    pub fn new(gtk_app: &Application, program: Rc<Program>) -> Self {
        let drawing = Rc::new(DrawingArea::new());

        let window = ApplicationWindow::builder()
            .application(gtk_app)
            .default_width(700)
            .default_height(500)
            .title("MutPaint")
            .show_menubar(true)
            .child(drawing.as_ref())
            .build();

        MainWindow {
            gtk_app: gtk_app.clone(),
            window,
            drawing: Rc::clone(&drawing),
            program: Rc::clone(&program),
        }
    }

    pub fn open_image(&self) {
        let window = self.window();
        let program = self.program.clone();
        let drawing = self.drawing.clone();

        let action = SimpleAction::new(actions::OPEN_IMAGE, None);
        action.connect_activate(clone!(
            #[strong]
            program,
            #[strong]
            drawing,
            move |_, _| {
                OpenImage::new().run(
                    &window,
                    clone!(
                        #[strong]
                        program,
                        #[strong]
                        drawing,
                        move |path| {
                            if let Some(path) = path {
                                let url = path.to_str().unwrap();
                                program.open_image(url);
                                drawing.queue_draw();
                            }
                        }
                    ),
                );
            }
        ));

        self.gtk_app.add_action(&action);
        self.gtk_app
            .set_accels_for_action(actions::app::OPEN_IMAGE, &["<Ctrl>O"]);
    }

    pub fn exit(&self) {
        let app = self.gtk_app.clone();
        let action = SimpleAction::new(actions::EXIT, None);
        action.connect_activate(clone!(
            #[strong]
            app,
            move |_, _| {
                app.quit();
            }
        ));

        self.gtk_app.add_action(&action);
        self.gtk_app
            .set_accels_for_action(actions::app::EXIT, &["<Ctrl>Q"]);
    }

    pub fn window(&self) -> ApplicationWindow {
        self.window.clone()
    }

    pub fn show(&self) {
        self.window.present();
    }

    pub fn start(&self) {
        self.connect_events();
        self.drawing_events();
        self.show();
    }

    pub fn connect_events(&self) {
        self.open_image();
        self.exit();
    }

    pub fn drawing_events(&self) {
        let motion = EventControllerMotion::new();
        let gesture = GestureClick::new();
        let scroll = EventControllerScroll::new(EventControllerScrollFlags::all());

        let program = self.program.clone();
        let drawing = self.drawing.clone();

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

        scroll.connect_scroll(clone!(
            #[strong]
            program,
            #[strong]
            drawing,
            move |_, _, y| {
                program.on_event(AppEvents::ScroolEvent { delta_y: y });
                if program.state.borrow().needs_paint() {
                    drawing.queue_draw();
                    program.state.borrow_mut().stop_request_paint();
                }
                return glib::Propagation::Proceed;
            }
        ));

        drawing.add_controller(motion);
        drawing.add_controller(gesture);
        drawing.add_controller(scroll);
        drawing.set_draw_func(clone!(
            #[strong]
            program,
            move |_, ctx, _, _| {
                program.draw(ctx);
            }
        ));
    }
}

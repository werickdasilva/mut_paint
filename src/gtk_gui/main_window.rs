use crate::{
    core::event::AppEvents,
    geometry::Point,
    gtk_gui::{actions, open_image::OpenImage},
    program::Program,
};
use gtk::{
    Application, ApplicationWindow, Button, CenterBox, DrawingArea, EventControllerMotion,
    EventControllerScroll, EventControllerScrollFlags, GestureClick, Label, Orientation,
    gio::{
        SimpleAction,
        prelude::{ActionMapExt, ApplicationExt},
    },
    glib::{self, clone},
    prelude::{BoxExt, DrawingAreaExtManual, GtkApplicationExt, GtkWindowExt, WidgetExt},
};
use std::rc::Rc;

pub struct MainWindow {
    gtk_app: Application,
    window: ApplicationWindow,
    drawing: Rc<DrawingArea>,
    program: Rc<Program>,
    label_zoom: Rc<Label>,
    label_rotate: Rc<Label>,
}

impl MainWindow {
    pub fn new(gtk_app: &Application, program: Rc<Program>) -> Self {
        let vbox = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .hexpand(true)
            .vexpand(true)
            .build();
        let label_zoom = Rc::new(Label::new(Some(&program.zoom_view().as_str())));
        let label_rotate = Rc::new(Label::new(Some(&program.rotate_view().as_str())));
        let header_bar = Self::make_header_bar(label_zoom.clone(), label_rotate.clone());

        let drawing = DrawingArea::builder().hexpand(true).vexpand(true).build();
        let drawing = Rc::new(drawing);

        vbox.append(&header_bar);
        vbox.append(drawing.as_ref());

        let window = ApplicationWindow::builder()
            .application(gtk_app)
            .default_width(700)
            .default_height(500)
            .title("MutPaint")
            .show_menubar(true)
            .child(&vbox)
            .build();

        MainWindow {
            gtk_app: gtk_app.clone(),
            window,
            drawing: Rc::clone(&drawing),
            program: Rc::clone(&program),
            label_zoom: label_zoom,
            label_rotate,
        }
    }

    fn make_header_bar(label: Rc<Label>, rotate: Rc<Label>) -> CenterBox {
        let btn_open_image = Button::builder()
            .icon_name("insert-image")
            .action_name(actions::app::OPEN_IMAGE)
            .build();

        let center_widget = gtk::Box::builder()
            .orientation(Orientation::Horizontal)
            .build();
        let btn_zoom_in = Button::builder()
            .icon_name("zoom-in")
            .action_name(actions::app::ZOOM_IN)
            .build();
        let btn_zoom_out = Button::builder()
            .icon_name("zoom-out")
            .action_name(actions::app::ZOOM_OUT)
            .build();
        let btn_rotate_left = Button::builder()
            .icon_name("object-rotate-left-symbolic")
            .tooltip_markup("Rotate Left")
            .action_name(actions::app::ROTATE_LEFT)
            .build();
        let btn_rotete_right = Button::builder()
            .icon_name("object-rotate-right-symbolic")
            .tooltip_markup("Rotate Right")
            .action_name(actions::app::ROTATE_RIGHT)
            .build();
        center_widget.append(&btn_zoom_in);
        center_widget.append(label.as_ref());
        center_widget.append(&btn_zoom_out);
        center_widget.append(&btn_rotate_left);
        center_widget.append(rotate.as_ref());
        center_widget.append(&btn_rotete_right);

        CenterBox::builder()
            .css_classes(["tool-bar"])
            .start_widget(&btn_open_image)
            .center_widget(&center_widget)
            .height_request(40)
            .build()
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
        self.register_zoom_action();
        self.register_rotate_action();
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

    pub fn register_zoom_action(&self) {
        self.on_register_action(
            actions::ZOOM_IN,
            &["<Ctrl>plus"],
            clone!(
                #[strong(rename_to = label_zoom)]
                self.label_zoom,
                move |program, drawing| {
                    program.zoom_in();
                    drawing.queue_draw();
                    label_zoom.set_label(program.zoom_view().as_str());
                }
            ),
        );
        self.on_register_action(
            actions::ZOOM_OUT,
            &["<Ctrl>minus"],
            clone!(
                #[strong(rename_to = label_zoom)]
                self.label_zoom,
                move |program, drawing| {
                    program.zoom_out();
                    drawing.queue_draw();
                    label_zoom.set_label(program.zoom_view().as_str());
                }
            ),
        );
    }

    fn register_rotate_action(&self) {
        self.on_register_action(
            actions::ROTATE_LEFT,
            &[],
            clone!(
                #[strong(rename_to = label_rotate)]
                self.label_rotate,
                move |program, drawing| {
                    program.rotate_left();
                    drawing.queue_draw();
                    label_rotate.set_label(program.rotate_view().as_str());
                }
            ),
        );
        self.on_register_action(
            actions::ROTATE_RIGHT,
            &[],
            clone!(
                #[strong(rename_to = label_rotate)]
                self.label_rotate,
                move |program, drawing| {
                    program.rotate_right();
                    drawing.queue_draw();
                    label_rotate.set_label(program.rotate_view().as_str());
                }
            ),
        );
    }

    pub fn on_register_action<F: Fn(Rc<Program>, Rc<DrawingArea>) + 'static>(
        &self,
        name: &str,
        accels: &[&str],
        f: F,
    ) {
        let action = SimpleAction::new(name, None);
        action.connect_activate(clone!(
            #[strong(rename_to = program)]
            self.program,
            #[strong(rename_to = drawing)]
            self.drawing,
            move |_, _| {
                f(program.clone(), drawing.clone());
            }
        ));

        self.gtk_app.add_action(&action);

        if !accels.is_empty() {
            self.gtk_app
                .set_accels_for_action(format!("app.{}", name).as_str(), accels);
        }
    }
}

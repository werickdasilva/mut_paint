use gtk::gio;
use mut_paint::{GtkGui, Program};
use std::rc::Rc;

fn main() {

    setup_resource();

    let program = Rc::new(Program::new());
    GtkGui::start(program.clone());
}

fn setup_resource() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");
}

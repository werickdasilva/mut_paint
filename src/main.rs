use mut_paint::{GtkGui, Program};
use std::rc::Rc;

fn main() {
    let program = Rc::new(Program::new());
    GtkGui::start(program.clone());
}

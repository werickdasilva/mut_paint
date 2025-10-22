use std::rc::Rc;
use mut_paint::{GtkGui, Program};


fn main() {
    let program = Rc::new(Program::new());
    GtkGui::start(program.clone());
}

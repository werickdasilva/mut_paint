use crate::gtk_gui::actions;
use gtk::{
    Application,
    gio::{Menu, MenuItem},
    prelude::GtkApplicationExt,
};

pub struct MenuBar {}

impl MenuBar {
    pub fn new(app: &Application) {
        let menu_bar = Menu::new();

        menu_bar.append_submenu(Some("File"), &Self::menu_file());

        app.set_menubar(Some(&menu_bar));
    }

    fn menu_file() -> Menu {
        let file = Menu::new();
        let open_file = MenuItem::new(Some("Open"), Some(actions::app::OPEN_IMAGE));
        let exit = MenuItem::new(Some("Exit"), Some(actions::app::EXIT));

        file.append_item(&open_file);
        file.append_item(&exit);

        file
    }
}

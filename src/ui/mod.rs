use gtk::{Application, gio, prelude::GtkApplicationExt};

pub fn create_menu_bar(app: &Application) {
    let menubar = gio::Menu::new();
    let filemenu = {
        let open_menu = gio::MenuItem::new(Some("Open"), Some("win.open-file"));
        let file_menu = gio::Menu::new();
        file_menu.append_item(&open_menu);
        file_menu
    };

    menubar.append_submenu(Some("File"), &filemenu);
    app.set_menubar(Some(&menubar));
}

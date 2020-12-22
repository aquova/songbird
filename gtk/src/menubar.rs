use gtk::prelude::*;
use gtk::{Menu, MenuBar, MenuItem};

pub struct EmuMenubar {
    pub open_btn: MenuItem,
    pub quit_btn: MenuItem,
    pub menubar: MenuBar,
}

impl EmuMenubar {
    pub fn new() -> Self {
        let menu_bar = MenuBar::new();

        // Create drop down menu, populate with items and append to menu bar
        let file_menu = Menu::new();
        let open = MenuItem::with_label("Open");
        let quit = MenuItem::with_label("Quit");
        file_menu.append(&open);
        file_menu.append(&quit);

        let file = MenuItem::with_label("File");
        file.set_submenu(Some(&file_menu));

        menu_bar.append(&file);

        Self {
            open_btn: open,
            quit_btn: quit,
            menubar: menu_bar,
        }
    }
}

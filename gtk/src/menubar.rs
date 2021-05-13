use gtk::prelude::*;
use gtk::{CheckMenuItem, Menu, MenuBar, MenuItem, RadioMenuItem};

pub const INIT_SCALE: usize = 3;
const MAX_SCALE: usize = 5;

pub struct EmuMenubar {
    pub open_btn: MenuItem,
    pub quit_btn: MenuItem,
    pub scale_btns: Vec<RadioMenuItem>,
    pub force_dmg: CheckMenuItem,
    pub menubar: MenuBar,
}

impl EmuMenubar {
    pub fn new() -> Self {
        let menu_bar = MenuBar::new();

        // Create drop down menu, populate with items and append to menu bar
        let file_menu = Menu::new();
        let file = MenuItem::with_label("File");
        file.set_submenu(Some(&file_menu));

        let open = MenuItem::with_label("Open");
        let quit = MenuItem::with_label("Quit");
        file_menu.append(&open);
        file_menu.append(&quit);
        menu_bar.append(&file);

        let settings_menu = Menu::new();
        let settings = MenuItem::with_label("Settings");
        settings.set_submenu(Some(&settings_menu));

        let scale_submenu = Menu::new();
        let scale = MenuItem::with_label("Scale");
        scale.set_submenu(Some(&scale_submenu));

        let btn_1x = RadioMenuItem::with_label("1x");
        for i in 1..=MAX_SCALE {
            let scale_btn = RadioMenuItem::with_label_from_widget(&btn_1x, Some(&format!("{}x", i)));
            if i == INIT_SCALE {
                scale_btn.set_active(true);
            }
            scale_submenu.append(&scale_btn);
        }

        let force_dmg = CheckMenuItem::with_label("Force DMG");

        settings_menu.append(&scale);
        settings_menu.append(&force_dmg);
        menu_bar.append(&settings);

        Self {
            open_btn: open,
            quit_btn: quit,
            scale_btns: btn_1x.get_group(),
            force_dmg: force_dmg,
            menubar: menu_bar,
        }
    }
}

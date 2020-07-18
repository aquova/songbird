use imgui::{MenuItem, Ui};

pub struct MenuState {

}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {}
    }

    pub fn create_menu(&mut self, ui: &Ui) {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu(im_str!("Menu"), true) {
                MenuItem::new(im_str!("Open ROM"))
                    .shortcut(im_str!("CTRL+O"))
                    .build(ui);
                MenuItem::new(im_str!("Quit"))
                    .shortcut(im_str!("CTRL+Q"))
                    .build(ui);
                menu.end(ui);
            }
            menu_bar.end(ui);
        }
    }
}

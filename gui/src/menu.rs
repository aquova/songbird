extern crate imgui_file_explorer;

use imgui::{ComboBox, MenuItem, Ui, Window};
use imgui_file_explorer::UiFileExplorer;

#[derive(Copy, Clone, PartialEq)]
pub enum Shaders {
    None,
    Greenscale,
}

pub struct MenuState {
    show_rom_dialog: bool,
    shader_index: usize,
    filename: Option<String>,
    load_required: bool,
}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {
            show_rom_dialog: false,
            shader_index: 0,
            filename: None,
            load_required: false,
        }
    }

    /// ```
    /// Create main menu bar
    ///
    /// Creates main file bar
    ///
    /// Input:
    ///     Imgui frame object (&Ui)
    /// ```
    pub fn create_menu(&mut self, ui: &Ui) {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            // Main menu
            if let Some(menu) = ui.begin_menu(im_str!("Menu"), true) {
                MenuItem::new(im_str!("Open ROM"))
                    .build_with_ref(ui, &mut self.show_rom_dialog);
                menu.end(ui);
            }
            // Appearance menu
            if let Some(menu) = ui.begin_menu(im_str!("Display"), true) {
                let items = [
                    im_str!("None"),
                    im_str!("Greenscale"),
                ];
                ComboBox::new(im_str!("Shader")).build_simple_string(ui, &mut self.shader_index, &items);
                menu.end(ui);
            }
            menu_bar.end(ui);
        }
    }

    /// ```
    /// Handle File Dialog
    ///
    /// Handles the 'Menu' drop down menu
    ///
    /// Input:
    ///     Imgui frame object (&Ui)
    /// ```
    pub fn handle_file_dialog(&mut self, ui: &Ui) {
        if self.show_rom_dialog {
            let mut new_file = None;

            Window::new(im_str!("Open ROM.."))
                .build(ui, || {
                    let file = ui.file_explorer("/", &["gb"]);
                    if let Ok(Some(file)) = file {
                        let str = file.into_os_string().into_string();
                        match str {
                            Ok(f) => {
                                // If user selected a file, close window and note filename
                                new_file = Some(f);
                            },
                            Err(_) => {
                                new_file = None;
                            }
                        }
                    }
                });

            // Update filepath and tell program it is time to load new ROM
            if new_file.is_some() {
                self.filename = new_file;
                self.show_rom_dialog = false;
                self.load_required = true;
            }
        }
    }

    pub fn handle_display_dialog(&self, _ui: &Ui) -> Shaders {
        let shaders = [
            Shaders::None,
            Shaders::Greenscale,
        ];
        shaders[self.shader_index]
    }

    /// ```
    /// Get ROM filename
    ///
    /// Returns the path to the currently loaded ROM
    ///
    /// Output:
    ///     Path to ROM file (&str)
    /// ```
    pub fn get_rom_filename(&self) -> &str {
        self.filename.as_ref().unwrap()
    }

    /// ```
    /// Set ROM filename
    ///
    /// Manually set the name of the ROM to open, indicate load required
    ///
    /// Input:
    ///     Filepath of ROM to open
    /// ```
    pub fn set_rom_filename(&mut self, filename: String) {
        self.filename = Some(filename);
        self.load_required = true;
    }

    /// ```
    /// Is load time?
    ///
    /// Is it time to load a new ROM?
    ///
    /// Output:
    ///     Whether to load ROM in self.filename (bool)
    /// ```
    pub fn is_load_time(&mut self) -> bool {
        let load_time = self.load_required;
        self.load_required = false;
        load_time
    }
}

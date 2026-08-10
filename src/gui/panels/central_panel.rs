use crate::gui::GUI;
use egui::Ui;

pub fn show(gui: &mut GUI, ui: &mut Ui) {
    let theme = &gui.theme;

    gui.argon2.show(theme, ui);

    gui.file_encryption_ui.show(theme, ui);
    gui.text_hashing_ui.show(theme, ui);

    gui.msg_window.show(theme, ui);
    gui.loading_window.show(theme, ui);

    gui.inject_elegance_theme(ui.ctx());
}

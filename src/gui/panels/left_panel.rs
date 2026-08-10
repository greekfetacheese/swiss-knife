use crate::gui::GUI;
use egui::{Margin, RichText, Stroke, Ui, vec2};

use zeus_widgets::Button;

pub fn show(gui: &mut GUI, ui: &mut Ui) {
    ui.set_width(140.0);

    let theme = &gui.theme;

    let color = theme.colors.hover;
    let stroke = Stroke::new(1.0, color);
    let frame = theme
        .frame2
        .inner_margin(Margin::symmetric(0, 10))
        .stroke(stroke);

    frame.show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().button_padding = vec2(8.0, 6.0);

            let text_size = gui.theme.text_sizes.normal;
            let button_size = vec2(80.0, 30.0);

            let is_open = gui.file_encryption_ui.is_open();
            let button = Button::selectable(is_open, RichText::new("Encrypt").size(text_size))
                .min_size(button_size);

            if ui.add(button).clicked() {
                gui.file_encryption_ui.open();
                gui.text_hashing_ui.close();
            }

            let is_open = gui.text_hashing_ui.is_open();
            let button = Button::selectable(is_open, RichText::new("Hash").size(text_size))
                .min_size(button_size);

            if ui.add(button).clicked() {
                gui.text_hashing_ui.open();
                gui.file_encryption_ui.close();
            }
        });
    });
}

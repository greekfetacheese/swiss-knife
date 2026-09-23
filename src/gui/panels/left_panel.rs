use crate::gui::GUI;
use egui::{RichText, Shadow, Stroke, Ui, vec2};

use egui_elements::{Frame as Frame2, Label};

pub fn show(gui: &mut GUI, ui: &mut Ui) {
    ui.set_width(140.0);

    let theme = &gui.theme;

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, theme.spacing.xs);

        let text_size = theme.typography.normal;

        let mut visuals = theme.frame2_visuals();
        visuals.bg = theme.frame1.fill;
        visuals.border = Stroke::NONE;
        visuals.shadow = Shadow::NONE;

        let frame = Frame2::from_egui(theme.frame2)
            .interactive(true)
            .fill_width(true)
            .visuals(visuals)
            .corner_radius(0);

        let is_open = gui.file_encryption_ui.is_open();
        let encrypt = frame.selected(is_open).show(ui, |ui| {
            ui.add(Label::new(RichText::new("Encrypt").size(text_size), None).interactive(false));
        });

        if encrypt.response.clicked() {
            gui.file_encryption_ui.open();
            gui.text_hashing_ui.close();
        }

        let is_open = gui.text_hashing_ui.is_open();
        let hash = frame.selected(is_open).show(ui, |ui| {
            ui.add(Label::new(RichText::new("Hash").size(text_size), None).interactive(false));
        });

        if hash.response.clicked() {
            gui.text_hashing_ui.open();
            gui.file_encryption_ui.close();
        }
    });
}

use crate::gui::SHARED_GUI;
use eframe::{
    CreationContext,
    egui::{CentralPanel, Context, Frame, Panel, Rgba, Ui, Visuals},
};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SwissKnifeApp {
    pub style_has_been_set: bool,
    allow_close: Arc<AtomicBool>,
    shutdown_started: bool,
}

impl SwissKnifeApp {
    pub fn new(cc: &CreationContext) -> Self {
        let theme = SHARED_GUI.read(|gui| gui.theme.clone());

        cc.egui_ctx.set_global_style(theme.style.clone());

        let app = Self {
            style_has_been_set: false,
            allow_close: Arc::new(AtomicBool::new(false)),
            shutdown_started: false,
        };

        app
    }

    fn on_shutdown(&mut self, ctx: &Context) {
        if !ctx.input(|i| i.viewport().close_requested()) {
            return;
        }

        // Final close after cleanup finished, do not cancel.
        if self.allow_close.load(Ordering::SeqCst) {
            return;
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);

        if self.shutdown_started {
            return;
        }

        self.shutdown_started = true;

        let allow_close = self.allow_close.clone();
        let egui_ctx = ctx.clone();

        std::thread::spawn(move || {
            SHARED_GUI.write(|gui| {
                gui.file_encryption_ui.credentials_form.erase();
                gui.text_hashing_ui.erase();
            });

            // Allow the next close_requested through, then re-request close.
            allow_close.store(true, Ordering::SeqCst);
            egui_ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            egui_ctx.request_repaint();
        });
    }
}

impl eframe::App for SwissKnifeApp {
    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        SHARED_GUI.write(|gui| {
            self.on_shutdown(ui.ctx());

            // This is needed for Windows
            if !self.style_has_been_set {
                let style = gui.theme.style.clone();
                ui.set_global_style(style);
                self.style_has_been_set = true;
            }

            let theme = &gui.theme;
            let bg_color = theme.colors.bg;
            let panel_frame = Frame::new().fill(bg_color);
            let top_frame = Frame::new().inner_margin(5).fill(bg_color);

            Panel::left("left_panel")
                .max_size(140.0)
                .resizable(false)
                .frame(top_frame)
                .show_separator_line(false)
                .show(ui, |ui| {
                    gui.show_left_panel(ui);
                });

            CentralPanel::default().frame(panel_frame).show(ui, |ui| {
                gui.show_central_panel(ui);
            });
        });
    }
}

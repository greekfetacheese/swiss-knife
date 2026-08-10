use egui::{Order, RichText, Spinner, Ui, vec2};

use zeus_theme::Theme;
use zeus_widgets::{Button, Modal};

pub struct LoadingWindow {
    open: bool,
    pub msg: String,
    pub size: (f32, f32),
}

impl LoadingWindow {
    pub fn new() -> Self {
        Self {
            open: false,
            msg: String::new(),
            size: (200.0, 100.0),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn open(&mut self, msg: impl Into<String>) {
        self.open = true;
        self.msg = msg.into();
    }

    pub fn reset(&mut self) {
        self.open = false;
        self.msg = String::new();
        self.size = (200.0, 100.0);
    }

    pub fn new_size(&mut self, size: (f32, f32)) {
        self.size = size;
    }

    pub fn show(&mut self, theme: &Theme, ui: &mut Ui) {
        if !self.open {
            return;
        }

        let mut open = self.open;

        Modal::new("Loading", &mut open)
            .closable(false)
            .backdrop_order(Order::Tooltip)
            .content_order(Order::Debug)
            .show(ui.ctx(), |ui| {
                ui.set_width(self.size.0);
                ui.set_height(self.size.1);
                ui.vertical_centered(|ui| {
                    ui.add(Spinner::new().size(50.0).color(theme.colors.text));
                    ui.label(RichText::new(&self.msg).size(17.0));
                });
            });
    }
}

pub struct MsgWindow {
    open: bool,
    pub message: String,
    pub size: (f32, f32),
}

impl MsgWindow {
    pub fn new() -> Self {
        Self {
            open: false,
            message: String::new(),
            size: (300.0, 300.0),
        }
    }

    /// Open the window with this title and message
    pub fn open(&mut self, msg: impl Into<String>) {
        self.open = true;
        self.message = msg.into();
    }

    pub fn reset(&mut self) {
        self.open = false;
    }

    pub fn show(&mut self, theme: &Theme, ui: &mut Ui) {
        if !self.open {
            return;
        }

        let msg = RichText::new(&self.message).size(theme.text_sizes.normal);
        let mut open = self.open;

        Modal::new("msg_window", &mut open)
            .closable(false)
            .backdrop_order(Order::Tooltip)
            .content_order(Order::Debug)
            .show(ui.ctx(), |ui| {
                ui.set_width(self.size.0);
                ui.set_max_height(self.size.1);

                ui.vertical_centered(|ui| {
                    ui.spacing_mut().item_spacing.y = 20.0;
                    ui.spacing_mut().button_padding = vec2(10.0, 8.0);

                    ui.label(msg);

                    let size = vec2(ui.available_width() * 0.5, 25.0);
                    let text = RichText::new("OK").size(theme.text_sizes.normal);
                    let visuals = theme.button_visuals();
                    let ok_button = Button::new(text).visuals(visuals).min_size(size);

                    if ui.add(ok_button).clicked() {
                        self.reset();
                    }
                });
            });
    }
}

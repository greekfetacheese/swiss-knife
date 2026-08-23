use egui::{Context, Order, Ui, vec2};
use egui_elements::{Theme, ThemeKind};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

pub mod app;
pub mod argon2;
pub mod file_encrypt;
pub mod hashing;
pub mod modals;
pub mod panels;

use argon2::Argon2Settings;
use file_encrypt::FileEncryptionUi;
use hashing::TextHashingUi;
use modals::*;

lazy_static! {
    pub static ref SHARED_GUI: SharedGUI = SharedGUI::default();
}

#[derive(Clone)]
pub struct SharedGUI(Arc<RwLock<GUI>>);

impl SharedGUI {
    /// Shared access to the [GUI]
    pub fn read<R>(&self, reader: impl FnOnce(&GUI) -> R) -> R {
        reader(&self.0.read().unwrap())
    }

    /// Exclusive mutable access to the [GUI]
    pub fn write<R>(&self, writer: impl FnOnce(&mut GUI) -> R) -> R {
        writer(&mut self.0.write().unwrap())
    }

    pub fn request_repaint(&self) {
        self.read(|gui| gui.request_repaint());
    }

    pub fn open_loading(&self, msg: impl Into<String>) {
        self.write(|gui| gui.loading_window.open(msg));
    }

    pub fn reset_loading(&self) {
        self.write(|gui| gui.loading_window.reset());
    }
}

impl Default for SharedGUI {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(GUI::default())))
    }
}

pub struct GUI {
    pub egui_ctx: Context,
    pub theme: Theme,
    pub argon2: Argon2Settings,
    pub file_encryption_ui: FileEncryptionUi,
    pub text_hashing_ui: TextHashingUi,
    pub loading_window: LoadingWindow,
    pub msg_window: MsgWindow,
}

impl Default for GUI {
    fn default() -> Self {
        let theme = Theme::new(ThemeKind::TokyoNight);

        Self {
            egui_ctx: Context::default(),
            theme,
            argon2: Argon2Settings::new(),
            file_encryption_ui: FileEncryptionUi::new(),
            text_hashing_ui: TextHashingUi::new(),
            loading_window: LoadingWindow::new(),
            msg_window: MsgWindow::new(),
        }
    }
}

impl GUI {
    pub fn open_msg_window(&mut self, msg: impl Into<String>) {
        self.msg_window.open(msg);
    }

    pub fn request_repaint(&self) {
        self.egui_ctx.request_repaint();
    }

    pub fn show_top_panel(&mut self, ui: &mut Ui) {
        panels::top_panel::show(self, ui);
    }

    pub fn show_left_panel(&mut self, ui: &mut Ui) {
        panels::left_panel::show(self, ui);
    }

    pub fn show_central_panel(&mut self, ui: &mut Ui) {
        panels::central_panel::show(self, ui);
    }
}

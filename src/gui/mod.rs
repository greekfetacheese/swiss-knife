use egui::{Color32, Context, Order, Ui, vec2};
use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use zeus_theme::{Theme, ThemeKind};

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

use elegance::Theme as EleganceTheme;

lazy_static! {
    pub static ref SHARED_GUI: SharedGUI = SharedGUI::default();
}

pub fn elegance_theme_key() -> egui::Id {
    egui::Id::new("elegance::theme")
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct EleganceThemeKey {
    dark: bool,
    bg: Color32,
    widget_bg: Color32,
    border: Color32,
    text: Color32,
    text_muted: Color32,
    accent: Color32,
    info: Color32,
    success: Color32,
    error: Color32,
    warning: Color32,
}

impl EleganceThemeKey {
    fn from_theme(theme: &Theme) -> Self {
        let c = &theme.colors;
        Self {
            dark: theme.dark_mode,
            bg: c.bg,
            widget_bg: c.widget_bg,
            border: c.border,
            text: c.text,
            text_muted: c.text_muted,
            accent: c.accent,
            info: c.info,
            success: c.success,
            error: c.error,
            warning: c.warning,
        }
    }
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
    injected_elegance_key: Option<EleganceThemeKey>,
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
            injected_elegance_key: None,
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

    pub fn inject_elegance_theme(&mut self, ctx: &egui::Context) {
        let key = EleganceThemeKey::from_theme(&self.theme);
        if self.injected_elegance_key == Some(key) {
            return;
        }

        let c = &self.theme.colors;
        let mut pal = if key.dark {
            elegance::Palette::charcoal()
        } else {
            elegance::Palette::frost()
        };

        pal.is_dark = key.dark;
        pal.bg = c.bg;
        pal.card = c.widget_bg;
        pal.input_bg = c.widget_bg;
        pal.border = c.border;
        pal.text = c.text;
        pal.text_muted = c.text_muted;
        pal.text_faint = c.text_muted;
        pal.focus = c.accent;
        pal.blue = c.info;
        pal.green = c.success;
        pal.green_hover = c.success;
        pal.red = c.error;
        pal.red_hover = c.error;
        pal.amber = c.warning;
        pal.amber_hover = c.warning;
        pal.purple = c.accent;
        pal.purple_hover = c.accent;
        pal.success = c.success;
        pal.danger = c.error;
        pal.warning = c.warning;

        let elegance_theme = EleganceTheme {
            palette: pal,
            ..EleganceTheme::slate()
        };

        ctx.data_mut(|d| d.insert_temp(elegance_theme_key(), elegance_theme));
        self.injected_elegance_key = Some(key);
    }
}

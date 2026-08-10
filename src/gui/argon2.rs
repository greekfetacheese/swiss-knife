use egui::{Order, RichText, Ui, vec2};
use elegance::{Badge, BadgeTone, Slider};
use ncrypt_me::Argon2;
use zeus_theme::Theme;
use zeus_widgets::{Button, Modal};

const MIN_M_COST: u32 = 64_000;
const MIN_T_COST: u32 = 8;
const MIN_P_COST: u32 = 1;

const MAX_M_COST: u32 = 16384_000;
const MAX_T_COST: u32 = 2048;
const MAX_P_COST: u32 = 64;

const M_COST_TIP: &str =
    "How much memory the Argon2 algorithm uses. Higher values are more secure but way slower, make sure the memory cost does not exceed your computer RAM.
    This is the most improtant parameter against GPU/ASIC brute-forcing attacks.
    You probably want to just increase the Memory cost to a sensible value 512 - 1024mb or even more if your RAM can afford it";

const T_COST_TIP: &str = "The number of iterations the Argon2 algorithm will run over the memory. Higher values are more secure but slower.";

const P_COST_TIP: &str = "How many parallel lanes (threads) the Argon2 algorithm will use.
You should keep this number as low as possible, best value for maximum security is 1";

pub struct Argon2Settings {
    open: bool,
    pub params: Argon2,
    pub size: (f32, f32),
}

impl Argon2Settings {
    pub fn new() -> Self {
        Self {
            open: false,
            params: Argon2::balanced(),
            size: (360.0, 300.0),
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn show(&mut self, theme: &Theme, ui: &mut Ui) {
        if !self.open {
            return;
        }

        let mut open = self.open;

        Modal::new("argon2_settings", &mut open)
            .closable(false)
            .backdrop_order(Order::Foreground)
            .content_order(Order::Tooltip)
            .show(ui.ctx(), |ui| {
                ui.set_width(self.size.0);
                ui.set_max_height(self.size.1);
                ui.spacing_mut().item_spacing = vec2(5.0, 15.0);
                ui.spacing_mut().button_padding = vec2(10.0, 4.0);

                let slider_size = vec2(ui.available_width() * 0.6, 20.0);
                let button_visuals = theme.button_visuals();

                let mem_fmt = |mb: f64| format!("{:.0}", mb / 1000.0);

                let q_mark = RichText::new("?").size(theme.text_sizes.normal);

                ui.vertical_centered(|ui| {
                    ui.allocate_ui(slider_size, |ui| {
                        ui.horizontal(|ui| {
                            let info_tip = Badge::new(q_mark.clone(), BadgeTone::Info);
                            ui.label(
                                RichText::new("Memory cost (MB):").size(theme.text_sizes.normal),
                            );
                            ui.add(info_tip).on_hover_text(M_COST_TIP);
                        });
                    });

                    ui.allocate_ui(slider_size, |ui| {
                        ui.add(
                            Slider::new(&mut self.params.m_cost, MIN_M_COST..=MAX_M_COST)
                                .value_fmt(mem_fmt),
                        );
                    });

                    ui.allocate_ui(slider_size, |ui| {
                        ui.horizontal(|ui| {
                            let info_tip = Badge::new(q_mark.clone(), BadgeTone::Info);
                            ui.label(RichText::new("Iterations:").size(theme.text_sizes.normal));
                            ui.add(info_tip).on_hover_text(T_COST_TIP);
                        });
                    });

                    ui.allocate_ui(slider_size, |ui| {
                        ui.add(Slider::new(
                            &mut self.params.t_cost,
                            MIN_T_COST..=MAX_T_COST,
                        ));
                    });

                    ui.allocate_ui(slider_size, |ui| {
                        ui.horizontal(|ui| {
                            let info_tip = Badge::new(q_mark, BadgeTone::Info);
                            ui.label(RichText::new("Parallelism:").size(theme.text_sizes.normal));
                            ui.add(info_tip).on_hover_text(P_COST_TIP);
                        });
                    });

                    ui.allocate_ui(slider_size, |ui| {
                        ui.add(Slider::new(
                            &mut self.params.p_cost,
                            MIN_P_COST..=MAX_P_COST,
                        ));
                    });

                    ui.add_space(20.0);

                    let size = vec2(ui.available_width() * 0.6, 35.0);
                    let text = RichText::new("OK").size(theme.text_sizes.normal);
                    let button = Button::new(text).visuals(button_visuals).min_size(size);

                    if ui.add(button).clicked() {
                        self.close();
                    }
                });
            });

        if !open {
            self.close();
        }
    }
}

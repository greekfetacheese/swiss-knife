use crate::gui::SHARED_GUI;
use eframe::egui::{
    Align, Align2, FontId, Frame, Layout, Margin, Order, RichText, Spinner, Ui, Window, vec2,
};
use ncrypt_me::secure_types::SecureString;
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use zeus_theme::Theme;
use zeus_ui_components::QrImage;
use zeus_widgets::{Button, ComboBox, Label, Modal, SecureTextEdit, Zeroize};

#[cfg(target_os = "linux")]
use zeus_ui_components::QRScanner;

#[derive(Clone, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
}

impl HashAlgorithm {
    pub fn to_string(&self) -> String {
        (match self {
            HashAlgorithm::Sha3_224 => "SHA3-224",
            HashAlgorithm::Sha3_256 => "SHA3-256",
            HashAlgorithm::Sha3_384 => "SHA3-384",
            HashAlgorithm::Sha3_512 => "SHA3-512",
        })
        .to_string()
    }

    pub fn to_vec(&self) -> Vec<HashAlgorithm> {
        vec![
            HashAlgorithm::Sha3_224,
            HashAlgorithm::Sha3_256,
            HashAlgorithm::Sha3_384,
            HashAlgorithm::Sha3_512,
        ]
    }
}

pub struct TextHashingUi {
    open: bool,
    show_qr: bool,
    qr_loading: bool,
    algorithm: HashAlgorithm,
    rounds: u64,
    pub input_text: SecureString,
    input_len: usize,
    pub output_hash: SecureString,
    output_qr: QrImage,
    #[cfg(target_os = "linux")]
    qr_scanner: QRScanner,
}

impl TextHashingUi {
    pub fn new() -> Self {
        Self {
            open: false,
            show_qr: false,
            qr_loading: false,
            algorithm: HashAlgorithm::Sha3_224,
            rounds: 0,
            input_text: SecureString::new_with_capacity(128).unwrap(),
            output_hash: SecureString::new_with_capacity(128).unwrap(),
            input_len: 0,
            output_qr: QrImage::empty_with_error("No QR code generated yet".to_string()),
            #[cfg(target_os = "linux")]
            qr_scanner: QRScanner::new(),
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

        self.show_qr(theme, ui);

        let mut open = self.open;

        Window::new("hashing_ui")
            .open(&mut open)
            .resizable(false)
            .collapsible(false)
            .order(Order::Background)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .title_bar(false)
            .frame(Frame::new())
            .show(ui.ctx(), |ui| {
                ui.set_width(600.0);
                ui.set_height(500.0);

                let mut should_calculate = false;

                ui.vertical_centered(|ui| {
                    ui.spacing_mut().item_spacing.y = 25.0;
                    ui.spacing_mut().button_padding = vec2(10.0, 8.0);

                    ui.add_space(10.0);

                    let size = vec2(300.0, 30.0);
                    ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 15.0;

                        self.select_algorithm(theme, ui);

                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 3.0;
                            let text = RichText::new("Rounds").size(theme.text_sizes.normal);
                            ui.label(text);

                            let rounds = self.rounds;
                            let mut rounds_str = rounds.to_string();

                            let res = SecureTextEdit::singleline(&mut rounds_str)
                                .visuals(theme.text_edit_visuals())
                                .desired_width(50.0)
                                .margin(Margin::same(5))
                                .font(FontId::proportional(theme.text_sizes.small))
                                .show(ui);

                            self.rounds = rounds_str.parse::<u64>().unwrap_or(0);

                            if res.response.changed() {
                                should_calculate = true;
                            }
                        });
                    });

                    ui.label(RichText::new("Input Text").size(theme.text_sizes.large));

                    let visuals = theme.text_edit_visuals();

                    let size = vec2(300.0, 300.0);
                    ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Min), |ui| {
                        self.input_text.secure_mut(|input_text| {
                            let text_edit = SecureTextEdit::multiline(input_text)
                                .visuals(visuals)
                                .desired_width(300.0)
                                .desired_rows(5)
                                .margin(Margin::same(10))
                                .font(FontId::proportional(theme.text_sizes.normal));
                            let output = text_edit.show(ui);
                            if output.response.changed() {
                                should_calculate = true;
                            }
                        });

                        #[cfg(target_os = "linux")]
                        {
                            let text = RichText::new("Scan QR Code").size(theme.text_sizes.small);
                            let button = Button::new(text).visuals(theme.button_visuals());
                            if ui.add(button).clicked() {
                                self.qr_scanner.open(ui.ctx().clone());
                            }

                            self.qr_scanner.show(ui.ctx());
                            let text_opt = self.qr_scanner.get_result();
                            if let Some(text) = text_opt {
                                self.qr_scanner.reset();
                                self.input_text = text;
                                should_calculate = true;
                            }
                        }
                    });

                    if should_calculate {
                        self.calculate_hash();
                    }

                    ui.label(RichText::new("Hash Output").size(theme.text_sizes.large));

                    self.output_hash.secure_mut(|output_hash| {
                        let text_edit = SecureTextEdit::multiline(output_hash)
                            .visuals(visuals)
                            .desired_width(300.0)
                            .desired_rows(5)
                            .margin(Margin::same(10))
                            .font(FontId::proportional(theme.text_sizes.normal));
                        text_edit.show(ui);
                    });

                    let size = vec2(ui.available_width() * 0.4, 30.0);

                    ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 20.0;

                        let btn_size = vec2(100.0, 30.0);
                        let visuals = theme.button_visuals();
                        let text = RichText::new("Copy").size(theme.text_sizes.normal);
                        let button = Button::new(text).visuals(visuals).min_size(btn_size);

                        if ui.add(button).clicked() {
                            self.output_hash.unlock_str(|text| {
                                ui.ctx().copy_text(text.to_owned());
                            })
                        }

                        let text = RichText::new("QR Code").size(theme.text_sizes.normal);
                        let button = Button::new(text).visuals(visuals).min_size(btn_size);

                        if ui.add(button).clicked() {
                            self.encode_qr();
                            self.show_qr = true;
                        }
                    });

                    if self.input_len == 0 {
                        self.output_hash.erase();
                    }
                });
            });
    }

    fn show_qr(&mut self, theme: &Theme, ui: &mut Ui) {
        let mut open = self.show_qr;

        Modal::new("qr_code_modal", &mut open)
            .closable(false)
            .backdrop_order(Order::Tooltip)
            .content_order(Order::Debug)
            .show(ui.ctx(), |ui| {
                ui.set_width(300.0);
                ui.set_max_height(300.0);

                ui.spacing_mut().item_spacing.y = 15.0;
                ui.spacing_mut().button_padding = vec2(10.0, 8.0);

                let frame = Frame::new().inner_margin(20);

                frame.show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        if self.qr_loading {
                            ui.add(Spinner::new().size(20.0));
                            return;
                        }

                        if let Some(err) = self.output_qr.error() {
                            ui.label(RichText::new(err.to_string()).size(theme.text_sizes.normal));
                            return;
                        }

                        let image = self.output_qr.image();
                        ui.add(image.fit_to_exact_size(vec2(250.0, 250.0)));

                        let text = RichText::new("Close").size(theme.text_sizes.normal);
                        let button = Button::new(text).visuals(theme.button_visuals());
                        if ui.add(button).clicked() {
                            self.show_qr = false;
                            let erased = self.output_qr.clear(ui.ctx());
                            debug_assert!(erased);
                        }
                    });
                });
            });
    }

    fn encode_qr(&mut self) {
        self.qr_loading = true;
        let output = self.output_hash.clone();

        std::thread::spawn(move || {
            let mut data = output.unlock_str(|output_hash| output_hash.to_owned());

            let uri = format!("output_hash_uri");
            let qr = QrImage::new(&data, uri);
            data.zeroize();

            SHARED_GUI.write(|gui| {
                gui.text_hashing_ui.output_qr = qr;
                gui.text_hashing_ui.qr_loading = false;
            });
        });
    }

    fn calculate_hash(&mut self) {
        let rounds = self.rounds;

        if rounds <= 256 {
            self.input_text.unlock_str(|input_text| {
                self.input_len = input_text.chars().count();

                if input_text.is_empty() {
                    return;
                }

                let output = hash(self.algorithm.clone(), input_text, self.rounds);

                self.output_hash = output.into();
            });
        } else {
            let input_text = self.input_text.clone();
            let algo = self.algorithm.clone();

            std::thread::spawn(move || {
                input_text.unlock_str(|input_text| {
                    let chars = input_text.chars().count();

                    SHARED_GUI.write(|gui| {
                        gui.text_hashing_ui.input_len = chars;
                    });

                    if chars == 0 {
                        return;
                    }

                    let output = hash(algo, input_text, rounds);

                    SHARED_GUI.write(|gui| {
                        gui.text_hashing_ui.output_hash = output.into();
                    });
                });
            });
        }
    }

    fn select_algorithm(&mut self, theme: &Theme, ui: &mut Ui) {
        let label_text = RichText::new(self.algorithm.to_string()).size(theme.text_sizes.normal);
        let label = Label::new(label_text, None);
        let visuals = theme.combo_box_visuals();

        ComboBox::new("select_algo", label)
            .visuals(visuals)
            .width(150.0)
            .show_ui(ui, |ui| {
                ui.spacing_mut().button_padding = vec2(5.0, 5.0);

                let mut algorithms = self.algorithm.to_vec();

                for selected_algorithm in algorithms.iter_mut() {
                    let value = ui.selectable_value(
                        &mut self.algorithm,
                        selected_algorithm.clone(),
                        RichText::new(selected_algorithm.to_string()).size(theme.text_sizes.normal),
                    );

                    if value.clicked() {
                        self.algorithm = selected_algorithm.clone();
                        self.calculate_hash();
                    }
                }
            });
    }
}

fn hash(algo: HashAlgorithm, input: &str, rounds: u64) -> String {
    let rounds = if rounds == 0 { 1 } else { rounds };

    match algo {
        HashAlgorithm::Sha3_224 => hash_rounds::<Sha3_224>(input, rounds),
        HashAlgorithm::Sha3_256 => hash_rounds::<Sha3_256>(input, rounds),
        HashAlgorithm::Sha3_384 => hash_rounds::<Sha3_384>(input, rounds),
        HashAlgorithm::Sha3_512 => hash_rounds::<Sha3_512>(input, rounds),
    }
}

fn hash_rounds<D: Digest>(input: &str, rounds: u64) -> String {
    let mut current = input.to_string();

    for _ in 0..rounds {
        current = hex::encode(D::digest(current.as_bytes()));
    }

    current
}

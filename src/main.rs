use crate::egui::Widget;
use eframe::egui::{CentralPanel, CtxRef, FontDefinitions, FontFamily, Ui, Vec2};
use eframe::epi::Frame;
use eframe::{egui, epi::App, run_native, NativeOptions};
use egui::Color32;
use regex::Regex;
use std::vec::Vec;

const PADDING: f32 = 20.0;
const ORANGE: Color32 = Color32::from_rgb(255, 140, 0);
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

struct Calculator {
    actions: Vec<ButtonsChar>,

    first_number: String,
    second_number: String,

    action: String,

    is_action_clicked: bool,
}

struct ButtonsChar {
    char: String,
}

impl Calculator {
    fn new() -> Calculator {
        let iter_char = ([" + ", " - ", " / ", " * "]).map(|var_for_char| ButtonsChar {
            char: var_for_char.to_string(),
        });
        Calculator {
            actions: Vec::from_iter(iter_char),
            first_number: String::from(""),
            second_number: String::from(""),
            action: String::from(""),
            is_action_clicked: false,
        }
    }

    fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 220.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 36.),
        );
        ctx.set_fonts(font_def);
    }
}

fn prepare_string(prev_value: &String, additional_value: &str) -> String {
    let result: String = String::from(prev_value);
    let intermediate_result = (result.clone() + additional_value).to_string();
    let re = Regex::new(r"^[1-9][0-9]{0,9}$").unwrap();

    if additional_value.len() != 1 {
        return result;
    }

    if re.is_match(&intermediate_result) {
        return intermediate_result;
    }

    return result;
}

fn top_label_panel(second_value: &String, first_value: &String, is_action: &bool) -> String {
    let first_number: String = String::from(first_value);
    let second_number = String::from(second_value);

    let label = if *is_action {
        second_number
    } else {
        first_number
    };
    return label;
}

fn equals_match(first_value: &String, second_value: &String, action: &String) -> f32 {
    let a: f32 = String::from(first_value).parse().unwrap();
    let b: f32 = String::from(second_value).parse().unwrap();

    let result = match action.as_str() {
        " + " => a + b,
        " - " => a - b,
        " * " => a * b,
        " / " => a / b,
        _ => a + a,
    };

    return result;
}

impl App for Calculator {
    fn update(&mut self, ctx: &CtxRef, _frame: &mut Frame<'_>) {
        egui::TopBottomPanel::top("label").show(ctx, |ui| {
            let label = top_label_panel(
                &self.second_number,
                &self.first_number,
                &self.is_action_clicked,
            );

            ui.horizontal(|ui| {
                let button_equals = egui::Button::new(" = ").text_color(WHITE).small();
                let response_equals = (button_equals).ui(ui);

                if response_equals.clicked() {
                    let result =
                        equals_match(&self.first_number, &self.second_number, &self.action);

                    self.first_number = result.to_string();
                    self.second_number = String::from("0");
                    self.action = String::from("");
                    self.is_action_clicked = false;
                }

                ui.colored_label(WHITE, label);
            });
            ui.add_space(PADDING);
        });
        egui::SidePanel::left("chars").show(ctx, |ui| {
            for a in &self.actions {
                let button_chars = egui::Button::new(&a.char)
                    .fill(ORANGE)
                    .text_color(WHITE)
                    .small();
                let response = button_chars.ui(ui);

                if response.clicked() {
                    if !self.is_action_clicked {
                        self.action = (a.char).clone();
                        self.is_action_clicked = true;
                    };
                }
            }
        });
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            let vec_digits = vec![
                vec!["7", "8", "9"],
                vec!["4", "5", "6"],
                vec!["1", "2", "3"],
                vec!["0"],
            ];

            for row in vec_digits {
                ui.horizontal(|ui| {
                    for item in row {
                        let button_equals = egui::Button::new(item).text_color(WHITE).small();
                        let response = button_equals.ui(ui);
                        if response.clicked() {
                            if self.is_action_clicked {
                                self.second_number = prepare_string(&self.second_number, &item);
                            } else {
                                self.first_number = prepare_string(&self.first_number, &item);
                            };
                        }
                    }
                });
            }
        });
    }

    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn name(&self) -> &str {
        "Calculator"
    }
}

fn main() {
    let app = Calculator::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(265., 240.));
    run_native(Box::new(app), win_option);
}

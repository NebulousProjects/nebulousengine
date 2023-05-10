use bevy::prelude::*;
use egui::*;
use nebulousengine_input::{*, types::*, enums::*};
use nebulousengine_utils::{is_of_var, from_enums::*};

pub struct InputEditor {
    pub handle: Handle<InputContainer>
}

impl InputEditor {
    pub fn ui(
        &mut self, ui: &mut egui::Ui, 
        inputs: &mut ResMut<Assets<InputContainer>>
    ) {

        let input_container = inputs.get_mut(&self.handle);
        if input_container.is_some() {
            // unpack
            let mut is_dirty = false;
            let input_container = input_container.unwrap();

            // render
            ui.vertical(|ui| {
                // add top bar
                ui.horizontal(|ui| {
                    if ui.button("Add Element").clicked() {
                        println!("TOOD add element")
                    }
                    if ui.button("Save").clicked() {
                        is_dirty = true;
                    }
                });

                // add list of collapsable inputs
                input_container.inputs.iter_mut().for_each(|(name, value)| {
                    ui.collapsing(name, |ui| {
                        // draw input basics
                        draw_input_basics(ui, value);

                        // draw descriptions header
                        ui.horizontal(|ui| {
                            ui.label("Input Options");
                            if ui.button("Add Option").clicked() {
                                println!("TODO add option");
                            }
                        });

                        // draw descriptions
                        value.descriptions.iter_mut().for_each(|desc| {
                            draw_input_option(ui, desc);
                        });
                    });
                });
            });

            // if json is marked dirty, save it
            if is_dirty {
                // println!("Saving: {} CONTENTS: {}", input_container.path, input_container.to_json());
                let result = std::fs::write(
                    format!("./assets/{}", input_container.path), 
                    input_container.to_json().to_string()
                );
                if result.is_err() {
                    error!("Input saved with error: {}", result.err().unwrap());
                }
            } 
        } else {
            ui.label("Loading...");
        } 
    }
}

fn draw_input_basics(ui: &mut egui::Ui, input: &mut InputValue) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label("Value: ");
            ui.label("Press Threshold: ");
        });
        ui.vertical(|ui| {
            let highlight = input.value.abs() >= input.press_threshold;

            ui.add(Slider::new(&mut input.value, -1.0 ..=1.0));
            ui.add(Slider::new(&mut input.press_threshold, 0.0 ..=1.0).trailing_fill(highlight));
        });
    });
}

fn draw_input_option(ui: &mut egui::Ui, opt: &mut InputDescription) {
    // create collapsable for description
    ui.collapsing(from_input_description_verbose(opt), |ui| {
        let is_single = is_of_var!(opt, InputDescription::Single);

        // option type dropdown
        ui.horizontal(|ui| {
            ui.label("Option Type: ");
            egui::ComboBox::from_label("")
                .selected_text(from_input_description(opt))
                .show_ui(ui, |ui| {
                    if ui.selectable_label(is_single, "Single").clicked() && !is_single {
                        println!("TODO Select Single")
                    }
                    if ui.selectable_label(!is_single, "Double").clicked() && is_single {
                        println!("TODO Select Double")
                    }
                });
        });

        // render option via match
        match opt {
            InputDescription::Single { input_type } => { draw_input_type(ui, input_type, "Input Type:", 1); }
            InputDescription::Double { positive_type, negative_type } => {
                draw_input_type(ui, positive_type, "Positive Input Type:", 1);
                draw_input_type(ui, negative_type, "Negative Input Type:", 2);
            }
        }
    });
}

fn draw_input_type(ui: &mut egui::Ui, input_type: &mut InputType, header: &str, id: u8) {
    // add input type dropdown
    ui.horizontal(|ui| {
        ui.label(header);
        egui::ComboBox::from_id_source(id)
            .selected_text(from_input_type(input_type))
            .show_ui(ui, |ui| {
                if ui.selectable_label(false, "Keyboard").clicked() { println!("TODO set input type"); }
                if ui.selectable_label(false, "Mouse Motion Horizontal").clicked() { println!("TODO set input type"); }
                if ui.selectable_label(false, "Mouse Motion Vertical").clicked() { println!("TODO set input type"); }
                if ui.selectable_label(false, "Mouse Button").clicked() { println!("TODO set input type"); }
                if ui.selectable_label(false, "Gamepad Button").clicked() { println!("TODO set input type"); }
                if ui.selectable_label(false, "Gamepad Axis").clicked() { println!("TODO set input type"); }
            });
    });

    // render input type via match
    match input_type {
        InputType::Keyboard(code) => {
            ui.horizontal(|ui| {
                ui.label("  Keycode: ");
                if ui.selectable_label(false, format!("{}", code.0)).clicked() {
                    println!("TODO change keycode");
                }
            });
        },
        InputType::MouseButton(button) => {
            ui.horizontal(|ui| {
                ui.label("  Mouse Button: ");
                if ui.selectable_label(false, from_mouse_button(button)).clicked() {
                    println!("TODO change mouse button");
                }
            });
        },
        InputType::GamepadButton(button) => {
            ui.horizontal(|ui| {
                ui.label("  Gamepad Button: ");
                if ui.selectable_label(false, from_gamepad_button_type(button)).clicked() {
                    println!("TODO change gamepad button");
                }
            });
        },
        InputType::GamepadAxis(axis) => {
            ui.horizontal(|ui| {
                ui.label("  Gamepad Axis: ");
                if ui.selectable_label(false, from_gamepad_axis_type(axis)).clicked() {
                    println!("TODO change gamepad axis");
                }
            });
        },
        InputType::MouseMotionX() => {},
        InputType::MouseMotionY() => {}
    }
}

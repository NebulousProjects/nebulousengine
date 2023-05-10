use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};
use egui::*;
use nebulousengine_input::{*, types::*, enums::*};
use nebulousengine_utils::{is_of_var, from_enums::*};

pub struct InputEditor {
    // basics
    pub handle: Handle<InputContainer>,
    pub selected_id: usize,
    pub is_dirty: bool,

    // input stuffs
    pub scancode: u32,
    pub name_entry_state: bool,
    pub name_entry: String,

    // remove stuff
    pub remove: Option<String>
}

impl InputEditor {
    pub fn ui(
        &mut self, ui: &mut egui::Ui, 
        inputs: &mut ResMut<Assets<InputContainer>>,
        key_events: &mut EventReader<KeyboardInput>
    ) {
        // update scancodes from key events
        for ev in key_events.iter() {
            match ev.state {
                ButtonState::Released => {
                    self.scancode = ev.scan_code;
                }
                _ => {}
            }
        }

        let input_container = inputs.get_mut(&self.handle);
        if input_container.is_some() {
            // unpack
            let input_container = input_container.unwrap();

            // remove
            if self.remove.is_some() {
                let to_remove = self.remove.as_ref().unwrap();
                input_container.inputs.remove(to_remove);
                self.is_dirty = true;
            }

            if self.name_entry_state {
                // ui.centered_and_justified(|ui| {
                    ui.vertical(|ui| {
                        // TextEdit::singleline(&mut self.name_entry).desired_width(0.2).show(ui);
                        ui.text_edit_singleline(&mut self.name_entry);
                        ui.horizontal(|ui| {
                            let cancel_button = egui::Button::new("Cancel").min_size(vec2(0.1, 0.0));
                            let add_button = egui::Button::new("Add").min_size(vec2(0.1, 0.0));

                            if ui.add(cancel_button).clicked() {
                                self.name_entry_state = false;
                            }
                            if ui.add(add_button).clicked() {
                                self.name_entry_state = false;
                                self.is_dirty = true;
                                input_container.inputs.insert(self.name_entry.clone(), InputValue::default());
                            }
                        });
                    });
                // });
            } else {
                // render
                ui.vertical(|ui| {

                    // add top bar
                    ui.horizontal(|ui| {
                        if ui.button("Add Element").clicked() {
                            self.name_entry_state = true;
                            self.name_entry = "".to_string();
                        }
                        if ui.button("Save").clicked() {
                            self.is_dirty = true;
                        }
                    });

                    // add list of collapsable inputs
                    input_container.inputs.iter_mut().for_each(|(name, value)| {
                        egui::CollapsingHeader::new(name).default_open(true).show(ui, |ui| {
                            // remove button
                            // if ui.button("Remove").clicked() {
                            //     self.remove = Some(name.clone());
                            // }
                            
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
                            value.descriptions.iter_mut().enumerate().for_each(|(id, desc)| {
                                draw_input_option(ui, self, desc, id);
                            });
                        });
                    });
                });

                // if json is marked dirty, save it
                if self.is_dirty {
                    // println!("Saving: {} CONTENTS: {}", input_container.path, input_container.to_json());
                    let result = std::fs::write(
                        format!("./assets/{}", input_container.path),
                        input_container.to_json().to_string()
                    );
                    if result.is_err() {
                        error!("Input saved with error: {}", result.err().unwrap());
                    }
                    self.is_dirty = false;
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

fn draw_input_option(ui: &mut egui::Ui, editor: &mut InputEditor, opt: &mut InputDescription, id: usize) {
    // create collapsable for description
    ui.collapsing(format!("{}: {}", id, from_input_description_verbose(opt)), |ui| {
        let is_single = is_of_var!(opt, InputDescription::Single);

        // option type dropdown
        ui.horizontal(|ui| {
            ui.label("Option Type: ");
            egui::ComboBox::from_id_source(id)
                .selected_text(from_input_description(opt))
                .show_ui(ui, |ui| {
                    if ui.selectable_label(is_single, "Single").clicked() && !is_single {
                        let _ = std::mem::replace(opt, InputDescription::Single { input_type: InputType::Keyboard(ScanCode(30)) });
                        editor.is_dirty = true;
                    }
                    if ui.selectable_label(!is_single, "Double").clicked() && is_single {
                        let _ = std::mem::replace(opt, InputDescription::Double { negative_type: InputType::Keyboard(ScanCode(30)), positive_type: InputType::Keyboard(ScanCode(32)) });
                        editor.is_dirty = true;
                    }
                });
        });

        // render option via match
        match opt {
            InputDescription::Single { input_type } => { draw_input_type(ui, editor, input_type, "Input Type:", id * 256 + 1); }
            InputDescription::Double { positive_type, negative_type } => {
                draw_input_type(ui, editor, positive_type, "Positive Input Type:", id * 256 + 1);
                draw_input_type(ui, editor, negative_type, "Negative Input Type:", id * 256 + 2);
            }
        }
    });
}

fn draw_input_type(ui: &mut egui::Ui, editor: &mut InputEditor, input_type: &mut InputType, header: &str, id: usize) {
    // add input type dropdown
    ui.horizontal(|ui| {
        ui.label(header);
        egui::ComboBox::from_id_source(id)
            .selected_text(from_input_type(input_type))
            .show_ui(ui, |ui| {
                if ui.selectable_label(false, "Keyboard").clicked() {
                    let _ = std::mem::replace(input_type, InputType::Keyboard(ScanCode(30)));
                    editor.is_dirty = true;
                }
                if ui.selectable_label(false, "Mouse Motion Horizontal").clicked() {
                    let _ = std::mem::replace(input_type, InputType::MouseMotionX());
                    editor.is_dirty = true;
                }
                if ui.selectable_label(false, "Mouse Motion Vertical").clicked() {
                    let _ = std::mem::replace(input_type, InputType::MouseMotionY());
                    editor.is_dirty = true;
                }
                if ui.selectable_label(false, "Mouse Button").clicked() {
                    let _ = std::mem::replace(input_type, InputType::MouseButton(MouseButton::Left));
                    editor.is_dirty = true;
                }
                if ui.selectable_label(false, "Gamepad Button").clicked() {
                    let _ = std::mem::replace(input_type, InputType::GamepadButton(GamepadButtonType::North));
                    editor.is_dirty = true;
                }
                if ui.selectable_label(false, "Gamepad Axis").clicked() {
                    let _ = std::mem::replace(input_type, InputType::GamepadAxis(GamepadAxisType::LeftStickX));
                    editor.is_dirty = true;
                }
            });
    });

    // render input type via match
    match input_type {
        InputType::Keyboard(code) => {
            ui.horizontal(|ui| {
                // check if selected
                let selected = id == editor.selected_id;
                
                // create ui
                ui.label("  Keycode: ");
                if ui.selectable_label(selected, format!("{}", code.0)).clicked() {
                    if !selected {
                        editor.selected_id = id;
                        editor.scancode = u32::MAX;
                    }
                }

                // if scancode changed and selected, update
                if selected && editor.scancode != u32::MAX {
                    let _ = std::mem::replace(code, ScanCode(editor.scancode));
                    editor.selected_id = usize::MAX;
                    editor.is_dirty = true;
                }
            });
        },
        InputType::MouseButton(button) => {
            ui.horizontal(|ui| {
                ui.label("  Mouse Button: ");
                egui::ComboBox::from_id_source(id + 10)
                    .selected_text(from_mouse_button_english(button))
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "Left").clicked() {
                            let _ = std::mem::replace(button, MouseButton::Left);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Right").clicked() {
                            let _ = std::mem::replace(button, MouseButton::Right);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Middle").clicked() {
                            let _ = std::mem::replace(button, MouseButton::Middle);
                            editor.is_dirty = true;
                        }
                    });
            });
        },
        InputType::GamepadButton(button) => {
            ui.horizontal(|ui| {
                ui.label("  Gamepad Button: ");
                egui::ComboBox::from_id_source(id + 10)
                    .selected_text(from_gamepad_button_type_english(button))
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "North").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::North);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "South").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::South);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "East").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::East);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "West").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::West);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "C").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::C);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Z").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::Z);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "L1").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::LeftTrigger);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "L2").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::LeftTrigger2);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "R1").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::RightTrigger);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "R2").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::RightTrigger2);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Select").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::Select);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Start").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::Start);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Mode").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::Mode);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "L3").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::LeftThumb);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "R3").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::RightThumb);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "DPad Up").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::DPadUp);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "DPad Down").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::DPadDown);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "DPad Left").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::DPadLeft);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "DPad Right").clicked() {
                            let _ = std::mem::replace(button, GamepadButtonType::DPadRight);
                            editor.is_dirty = true;
                        }
                    });
            });
        },
        InputType::GamepadAxis(axis) => {
            ui.horizontal(|ui| {
                ui.label("  Gamepad Axis: ");
                egui::ComboBox::from_id_source(id + 10)
                    .selected_text(from_gamepad_axis_type_english(axis))
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "Left Stick Horizontal").clicked() {
                            let _ = std::mem::replace(axis, GamepadAxisType::LeftStickX);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Left Stick Vertical").clicked() {
                            let _ = std::mem::replace(axis, GamepadAxisType::LeftStickY);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Left Z").clicked() {
                            let _ = std::mem::replace(axis, GamepadAxisType::LeftZ);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Right Stick Horizontal").clicked() {
                            let _ = std::mem::replace(axis, GamepadAxisType::RightStickX);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Right Stick Vertical").clicked() {
                            let _ = std::mem::replace(axis, GamepadAxisType::RightStickY);
                            editor.is_dirty = true;
                        }
                        if ui.selectable_label(false, "Right Z").clicked() {
                            let _ = std::mem::replace(axis, GamepadAxisType::RightZ);
                            editor.is_dirty = true;
                        }
                    });
            });
        },
        InputType::MouseMotionX() => {},
        InputType::MouseMotionY() => {}
    }
}

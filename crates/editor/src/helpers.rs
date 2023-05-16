use egui::Color32;
use json::{JsonValue, array};
use nebulousengine_utils::optionals::optional_f32_radix;

pub fn edit_f32(ui: &mut egui::Ui, json: &mut JsonValue, index: usize, default: f32) -> bool {
    let value = json[index].as_f32().unwrap_or(default);
    // convert the input value to a string, and add a decimal if the number does not already have one
    let mut string = if value % 1.0 != 0.0 { value.clone().to_string() } else { format!("{}.0", value.clone()) };

    // add a text edit box to edit that string
    let response = ui.add_sized(egui::Vec2 { x: 60.0, y: 20.0 }, egui::TextEdit::singleline(&mut string));

    // if the string changed, attempt to parse it back into a f32
    if response.changed() {
        let result = string.parse::<f32>();

        // if the parse suceeded, pass the value back
        if result.is_ok() {
            json[index] = result.unwrap().into();
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn edit_f32_at_str(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, default: f32) -> bool {
    let mut is_dirty = false;
    let value = json[index].as_f32().unwrap_or(default);
    // convert the input value to a string, and add a decimal if the number does not already have one
    let mut string = if value % 1.0 != 0.0 { value.clone().to_string() } else { format!("{}.0", value.clone()) };

    ui.horizontal(|ui| {
        ui.label(index);
        // add a text edit box to edit that string
        let response = ui.add_sized(egui::Vec2 { x: 60.0, y: 20.0 }, egui::TextEdit::singleline(&mut string));

        // if the string changed, attempt to parse it back into a f32
        is_dirty = if response.changed() {
            let result = string.parse::<f32>();

            // if the parse suceeded, pass the value back
            if result.is_ok() {
                json[index] = result.unwrap().into();
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    is_dirty
}

pub fn edit_u32(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, default: u32) -> bool {
    let mut is_dirty = false;
    let value = json[index].as_u32().unwrap_or(default);
    // convert the input value to a string, and add a decimal if the number does not already have one
    let mut string = value.clone().to_string();

    ui.horizontal(|ui| {
        ui.label(index);
        // add a text edit box to edit that string
        let response = ui.add_sized(egui::Vec2 { x: 60.0, y: 20.0 }, egui::TextEdit::singleline(&mut string));

        // if the string changed, attempt to parse it back into a f32
        is_dirty = if response.changed() {
            let result = string.parse::<u32>();

            // if the parse suceeded, pass the value back
            if result.is_ok() {
                json[index] = result.unwrap().into();
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    is_dirty
}

pub fn edit_usize(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, default: usize) -> bool {
    let mut is_dirty = false;
    let value = json[index].as_usize().unwrap_or(default);
    // convert the input value to a string, and add a decimal if the number does not already have one
    let mut string = value.clone().to_string();

    ui.horizontal(|ui| {
        ui.label(index);
        // add a text edit box to edit that string
        let response = ui.add_sized(egui::Vec2 { x: 60.0, y: 20.0 }, egui::TextEdit::singleline(&mut string));

        // if the string changed, attempt to parse it back into a f32
        is_dirty = if response.changed() {
            let result = string.parse::<usize>();

            // if the parse suceeded, pass the value back
            if result.is_ok() {
                json[index] = result.unwrap().into();
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    is_dirty
}

pub fn edit_path(ui: &mut egui::Ui, json: &mut JsonValue, index: &str) -> bool {
    let mut is_dirty = false;

    // get string from json and check if it exists as a valid path
    let mut string = json[index].as_str().unwrap_or("").to_string();
    let exists = std::path::Path::new(&format!("./assets/{}", string)).exists();

    ui.vertical(|ui| {
        // create text edit and add it to the UI
        let textedit = egui::TextEdit::singleline(&mut string).text_color(if exists { Color32::GREEN } else { Color32::RED });
        let response = ui.add( textedit);

        // if path doesn't exist, add label saying so
        if !exists {
            let error_label = egui::RichText::new("Path does not exist!").color(Color32::RED);
            ui.label(error_label);
        }

        // mark is diry if the response changed
        is_dirty = is_dirty || response.changed();
    });

    // if is dirty, update json and return true
    if is_dirty {
        json[index] = string.into();
        true
    } else {
        false
    }
}

pub fn edit_vec2(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, default_x: f32, default_y: f32) -> bool {
    // make sure index exists in json
    if !json.has_key(index) { let _ = json.insert(index, array![default_x, default_y]); }
    let json = &mut json[index];

    // edit boxes and return the results
    edit_f32(ui, json, 0, default_x) || edit_f32(ui, json, 1, default_y)
}

pub fn edit_vec3(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, default_x: f32, default_y: f32, default_z: f32) -> bool {
    // make sure index exists in json
    if !json.has_key(index) { let _ = json.insert(index, array![default_x, default_y, default_z]); }
    let json = &mut json[index];

    // edit boxes and return the results
    edit_f32(ui, json, 0, default_x) || edit_f32(ui, json, 1, default_y) || edit_f32(ui, json, 2, default_z)
}

pub fn edit_bool(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, default: bool) -> bool {
    // make sure index exists in json
    if !json.has_key(index) { let _ = json.insert(index, default); }

    // make checkbox
    let mut checkbox = json[index].as_bool().unwrap_or(default);
    if ui.checkbox(&mut checkbox, index).changed() {
        let _ = json.insert(index, checkbox);
        true
    } else { false }
}

pub fn edit_enum_dropdown(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, options: &[&str], default: &str) -> bool {
    let mut is_dirty = false;
    
    // make sure enum exists in json
    if !json.has_key(index) { let _ = json.insert(index, default); }

    // create labelled drop down
    ui.horizontal(|ui| {
        // get current
        let current = json[index].as_str().unwrap();

        // label
        ui.label(index);

        // drop down
        egui::ComboBox::from_id_source(index)
            .selected_text(current)
            .show_ui(ui, |ui| {
                for option in options {
                    if ui.selectable_label(false, *option).clicked() {
                        let _ = json.insert(index, *option);
                        is_dirty = true;
                    }
                }
            });
    });

    is_dirty
}

pub fn edit_slider(ui: &mut egui::Ui, json: &mut JsonValue, index: &str, min: f32, max: f32, default: f32) -> bool {
    let mut is_dirty = false;

    // make sure index exists
    if !json.has_key(index) { let _ = json.insert(index, default); }

    // add slider with label
    ui.horizontal(|ui| {
        let mut value = json[index].as_f32().unwrap();

        ui.label(index);
        if ui.add(egui::Slider::new(&mut value, min ..=max)).changed() {
            let _ = json.insert(index, value);
            is_dirty = true;
        }
    });

    is_dirty
}

pub fn edit_color(ui: &mut egui::Ui, json: &mut JsonValue, index: &str) -> bool {
    let mut is_dirty = false;

    // make sure index exists in json
    if !json.has_key(index) { let _ = json.insert(index, "#FFFFFF"); }

    // convert original to a f32 array
    let original = json[index].as_str().unwrap();
    let mut color: [f32; 3] = if original.len() == 7 {
        [
            optional_f32_radix(&original[1..3], 16, 0.0) / 255.0, 
            optional_f32_radix(&original[3..5], 16, 0.0) / 255.0, 
            optional_f32_radix(&original[5..7], 16, 0.0) / 255.0
        ]
    } else if original.len() == 4 {
        [
            optional_f32_radix(&original[1..2], 16, 0.0) / 255.0, 
            optional_f32_radix(&original[2..3], 16, 0.0) / 255.0, 
            optional_f32_radix(&original[3..4], 16, 0.0) / 255.0
        ]
    } else { [1.0, 1.0, 1.0] };
    
    ui.horizontal(|ui| {
        ui.label(index);
        if ui.color_edit_button_rgb(&mut color).changed() {
            let _ = json.insert(
                index, 
                format!(
                    "#{:02X}{:02X}{:02X}", 
                    (color[0] * 255.0) as u32, 
                    (color[1] * 255.0) as u32, 
                    (color[2] * 255.0) as u32,
                )
            );
            is_dirty = true;
        }
    });

    is_dirty
}
use egui::Color32;
use json::JsonValue;

pub fn edit_f32(ui: &mut egui::Ui, json: &mut JsonValue, index: usize) -> bool {
    let value = json[index].as_f32().unwrap_or(0.0);
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
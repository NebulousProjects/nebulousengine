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
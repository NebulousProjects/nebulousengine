// struct for text elements
pub struct TextContainer {
    pub text: String
}

impl TextContainer {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    // .layouter(&mut layouter),
            );
        });
    }
}

// pub fn draw_text_editor(tab: &EditorTab, text: &TextContainer) {
//     egui::TextEdit::multiline(text)
//         .font(egui::TextStyle::Monospace) // for cursor height
//         .code_editor()
//         .desired_rows(10)
//         .lock_focus(true)
//         .desired_width(f32::INFINITY);
//         // .layouter(&mut layouter);
// }
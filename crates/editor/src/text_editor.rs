use std::{fs::File, path::PathBuf, io::Write};
use bevy::prelude::*;

// struct for text elements
pub struct TextContainer {
    pub text: String
}

// give the text container a function to draw
impl TextContainer {
    pub fn ui(&mut self, ui: &mut egui::Ui, path: &PathBuf) {
        // create a scroll area
        egui::ScrollArea::vertical().show(ui, |ui| {
            // add text edit area
            let text_area = ui.add_sized(ui.available_size(), 
                egui::TextEdit::multiline(&mut self.text)
                    .font(egui::TextStyle::Monospace) 
                    .code_editor()
                    // .desired_rows(100)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
            );

            // if the text area changes, save
            if text_area.changed() {
                let file = File::create(path);
                // based on the file create result, attempt write or print error
                match file {
                    Ok(mut file) => {
                        // attempt to write to the file
                        let write = self.text.as_str().as_bytes();
                        let write_result = file.write_all(write);

                        // if the file save fails, print the error
                        if write_result.is_err() {
                            error!("File save failed from file write with error {}", write_result.err().unwrap());
                        }
                    },
                    Err(error) => {
                        error!("File save failed from file create with error {}", error);
                    }
                }
                // if file.is_ok() {
                //     let write = self.text.as_str().as_bytes();
                //     file.unwrap().write_all(write);
                // } else {
                //     println!("Save failed from file create with error {}", file.err().unwrap());
                // }
            }
        });
    }
}
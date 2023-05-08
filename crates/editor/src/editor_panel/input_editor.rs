use std::{path::PathBuf};
use bevy::prelude::error;
use egui::ScrollArea;
use json::JsonValue;
use nebulousengine_utils::optionals::optional_string;

pub struct InputEditor {
    pub json: JsonValue
}

impl InputEditor {
    pub fn ui(&mut self, ui: &mut egui::Ui, path: &PathBuf) {
        let mut is_dirty = false;

        // setup contaiiner
        ui.vertical(|ui| {
            // add top button bar
            ui.horizontal(|ui| {
                if ui.button("+ Add Element").clicked() {
                    let result = self.json.push(json::parse("{}").unwrap());
                    if result.is_err() {
                        error!("Json push in input failed with error: {}", result.err().unwrap());
                    } else {
                        is_dirty = true;
                    }
                }
                if ui.button("Save").clicked() {
                    is_dirty = true;
                }
            });

            // add scroll area for input elements
            ScrollArea::vertical().show(ui, |ui| {
                if self.json.is_array() {
                    // loop through all elements
                    for i in 0 .. self.json.len() {
                        // get json and unpack
                        let json = &mut self.json[i];
                        let mut name = optional_string(json, "name").to_string();

                        // create collapsable
                        ui.collapsing(format!("Input Element {}", i), |ui| {
                            // add name editable
                            ui.horizontal(|ui| {
                                ui.label("Tag: ");
                                if ui.text_edit_singleline(&mut name).changed() {
                                    let result = json.insert("name", name);
                                    if result.is_err() {
                                        error!("Json name insert failed in input editor: {}", result.err().unwrap());
                                    } else {
                                        is_dirty = true;
                                    }
                                }
                            });
                        });
                    }
                } else {
                    self.json = json::parse("[]").unwrap();
                    is_dirty = true;
                }
            });
        });

        // if json is marked dirty, save it
        if is_dirty {
            let result = std::fs::write(path, self.json.to_string());
            if result.is_err() {
                error!("Json input save failed with error: {}", result.err().unwrap());
            }
        }
    }
}
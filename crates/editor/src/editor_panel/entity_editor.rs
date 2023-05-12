use bevy::prelude::*;

pub struct EntityEditor {

}

impl EntityEditor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn select(&mut self) {}
    pub fn deselect(&mut self) {}
    pub fn close(&mut self) {}

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO");
    }
}
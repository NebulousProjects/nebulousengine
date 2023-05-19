use bevy::prelude::EventWriter;
use egui::Ui;
use std::fs::{self, ReadDir, DirEntry};

use crate::EditorOpenFileEvent;

pub fn render_files(ui: &mut Ui, tabs: &mut EventWriter<EditorOpenFileEvent>) {
    let master_path = fs::read_dir("./assets").unwrap(); // todo file watching and caching

    // create scroll area for the files
    egui::ScrollArea::both().show(ui, |ui| {
        // render the contents of the master path
        render_directory_contents(ui, tabs, master_path);
    });
}

fn render_directory_contents(ui: &mut Ui, tabs: &mut EventWriter<EditorOpenFileEvent>, root: ReadDir) {
    // loop through contents of the directory and divide it into directories and files
    let (dirs, files): (Vec<_>, Vec<_>) = root.into_iter()
        .map(|p| { p.unwrap() })
        .partition(|entry| { entry.file_type().unwrap().is_dir() });

    // render all directories and then files
    for dir in dirs {
        render_directory(ui, tabs, dir);
    }
    for file in files {
        render_file(ui, tabs, file);
    }
}

fn render_file(ui: &mut Ui, events: &mut EventWriter<EditorOpenFileEvent>, path: DirEntry) {
    // add a button to the file that when clicked, run code
    // let response = ui.add(egui::widgets::Label::new(path.file_name().to_str().unwrap()).wrap(false).sense(egui::Sense::click()));
    // if response.clicked_by(egui::PointerButton::Primary) {
    //     events.send(EditorOpenFileEvent { path: path.path() })
    // }
    // if response.clicked_by(egui::PointerButton::Secondary) {
    //     println!("Released");
    // }
    egui::menu::menu_button(ui, path.file_name().to_str().unwrap(), |ui| {
        if ui.button("Open...").clicked() {
            events.send(EditorOpenFileEvent { path: path.path() })
        }
        if ui.button("Delete...").clicked() {
            let _ = std::fs::remove_file(path.path());
        }
    });
}

fn render_directory(ui: &mut Ui, tabs: &mut EventWriter<EditorOpenFileEvent>, dir: DirEntry) {
    // render collapsable containing directory contents
    ui.collapsing(dir.file_name().to_str().unwrap(), |ui| {
        render_directory_contents(ui, tabs, fs::read_dir(dir.path()).unwrap());
    });
}
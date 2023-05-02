use egui::Ui;
use std::fs::{self, ReadDir, DirEntry};

use crate::{editor_panel::*, text_editor::TextContainer};

pub fn render_files(ui: &mut Ui, tabs: &mut EditorTabs) {
    let master_path = fs::read_dir("./assets").unwrap(); // todo file watching and caching

    // create scroll area for the files
    egui::ScrollArea::both().show(ui, |ui| {
        // render the contents of the master path
        render_directory_contents(ui, tabs, master_path);
    });
}

fn render_directory_contents(ui: &mut Ui, tabs: &mut EditorTabs, root: ReadDir) {
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

fn render_file(ui: &mut Ui, tabs: &mut EditorTabs, path: DirEntry) {
    // add a button to the file that when clicked, run code
    if ui.add(egui::widgets::Label::new(path.file_name().to_str().unwrap()).wrap(false).sense(egui::Sense::click())).clicked() {
        // attempt to load the file as text, other, default ot unknown
        let input_str = std::fs::read_to_string(path.path());
        let editor_type = match input_str {
            Ok(str) => EditorTabType::Text(TextContainer { text: str }),
            Err(_) => EditorTabType::Unknown
        };
        
        // add tab
        tabs.tree.push_to_focused_leaf(EditorTab {
            path: path.path(),
            name: path.file_name().to_str().unwrap_or("").to_string(),
            tab_type: editor_type
        });
    }
}

fn render_directory(ui: &mut Ui, tabs: &mut EditorTabs, dir: DirEntry) {
    // render collapsable containing directory contents
    ui.collapsing(dir.file_name().to_str().unwrap(), |ui| {
        render_directory_contents(ui, tabs, fs::read_dir(dir.path()).unwrap());
    });
}
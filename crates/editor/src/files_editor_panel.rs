use egui::Ui;
use std::fs::{self, ReadDir, DirEntry};

pub fn render_files(ui: &mut Ui) {
    let master_path = fs::read_dir("./").unwrap(); // todo file watching and caching

    // create scroll area for the files
    egui::ScrollArea::both().show(ui, |ui| {
        // render the contents of the master path
        render_directory_contents(ui, master_path);
    });
}

fn render_directory_contents(ui: &mut Ui, root: ReadDir) {
    // loop through contents of the directory and divide it into directories and files
    let (dirs, files): (Vec<_>, Vec<_>) = root.into_iter()
        .map(|p| { p.unwrap() })
        .partition(|entry| { entry.file_type().unwrap().is_dir() });

    // render all directories and then files
    for dir in dirs {
        render_directory(ui, dir);
    }
    for file in files {
        render_file(ui, file);
    }
}

fn render_file(ui: &mut Ui, path: DirEntry) {
    //if ui.label(path.file_name().to_str().unwrap()).sense(Sense::click()).clicked() {
    if ui.add(egui::widgets::Label::new(path.file_name().to_str().unwrap()).wrap(false).sense(egui::Sense::click())).clicked() {
        println!("TODO open file");
    }
}

fn render_directory(ui: &mut Ui, dir: DirEntry) {
    // render collapsable containing directory contents
    ui.collapsing(dir.file_name().to_str().unwrap(), |ui| {
        render_directory_contents(ui, fs::read_dir(dir.path()).unwrap());
    });
}
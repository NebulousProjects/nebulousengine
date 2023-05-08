use std::{path::PathBuf};

use bevy::prelude::*;
use bevy_egui::EguiContexts;
use egui::{ScrollArea, Color32};
use nebulousengine_input::InputContainer;

use crate::text_editor::*;

use self::{image_viewer::ImageRenderer, input_editor::InputEditor};

pub mod text_editor;
pub mod image_viewer;
pub mod input_editor;

#[derive(Resource)]
pub struct EditorTabs {
    pub tree: Vec<EditorTab>,
    pub selected: usize
}

impl EditorTabs {
    pub fn new() -> Self {
        Self { tree: Vec::new(), selected: 0 }
    }
}

pub struct EditorTab {
    pub path: PathBuf,
    pub name: String,
    pub tab_type: EditorTabType
}

pub enum EditorTabType {
    Text(TextContainer),
    Image(ImageRenderer),
    Input(InputEditor),
    Unknown
}

// render editor in the center panel by a dock area
pub fn render_editor(
    mut contexts: EguiContexts, 
    tabs: &mut EditorTabs,

    images: Res<Assets<Image>>,
    inputs: Res<Assets<InputContainer>>
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        egui::TopBottomPanel::top("top_tab_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                // create a horizontal scroll area for all of the buttons
                ScrollArea::horizontal().show(ui, |ui| {
                    for i in 0 .. tabs.tree.len() {
                        // if i exceedes the trees limit, break
                        if i >= tabs.tree.len() { break; }

                        // horizontally stack the tabs button and its remove button
                        ui.horizontal(|ui| {
                            let tab = &tabs.tree[i];

                            // add open button with custom fill
                            let color = if tabs.selected == i { Color32::DARK_GREEN } else { Color32::BLACK };
                            let open_button = egui::Button::new(&tab.name).fill(color);
                            if ui.add(open_button).clicked() { tabs.selected = i.clone(); }

                            if ui.button("x").clicked() {
                                tabs.tree.remove(i);
                            }
                        });
                    }
                });
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // let tab = tabs.selected;
            if tabs.selected >= tabs.tree.len() {
                if tabs.tree.len() == 0 { return; }
                else { tabs.selected = 0; }
            }
            let tab = &mut tabs.tree[tabs.selected];

            // match to ui render function
            let tab_type = &mut tab.tab_type;
            match tab_type {
                EditorTabType::Text(text) => text.ui(ui, &tab.path),
                EditorTabType::Image(image) => image.ui(ui, &ui.max_rect(), &images),
                EditorTabType::Input(input) => input.ui(ui, &inputs),
                EditorTabType::Unknown => draw_unknown(ui, tab)
            };
        })
    });
}

fn draw_unknown(ui: &mut egui::Ui, tab: &EditorTab) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Unknown file type for file {}", tab.name));
    });
}
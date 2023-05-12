use std::{path::PathBuf};

use bevy::{prelude::*, input::keyboard::KeyboardInput};
use bevy_egui::EguiContexts;
use egui::{ScrollArea, Color32};
use nebulousengine_input::*;
use nebulousengine_utils::*;

use crate::text_editor::*;

use self::{image_viewer::ImageRenderer, input_editor::InputEditor, model_viewer::ModelViewer};

pub mod text_editor;
pub mod image_viewer;
pub mod input_editor;
pub mod model_viewer;

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
    Model(ModelViewer),
    Unknown
}

// render editor in the center panel by a dock area
pub fn render_editor(
    contexts: &mut EguiContexts, 
    tabs: &mut EditorTabs,
    rendered_texture_id: Local<egui::TextureId>,

    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    mut inputs: ResMut<Assets<InputContainer>>,
    mut key_events: EventReader<KeyboardInput>,
    mut viewport: ResMut<ViewportContainer>,
) {
    // egui::CentralPanel::default().show(context, |ui| {
        egui::TopBottomPanel::top("top_tab_bar").show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                // create a horizontal scroll area for all of the buttons
                ScrollArea::horizontal().show(ui, |ui| {
                    for i in 0 .. tabs.tree.len() {
                        // if i exceedes the trees limit, break
                        if i >= tabs.tree.len() { break; }

                        // horizontally stack the tabs button and its remove button
                        ui.horizontal(|ui| {
                            let tree = &mut tabs.tree;
                            let tab = tree.get(i).unwrap();

                            // add open button with custom fill
                            let color = if tabs.selected == i { Color32::DARK_GREEN } else { Color32::BLACK };
                            let open_button = egui::Button::new(&tab.name).fill(color);
                            if ui.add(open_button).clicked() {
                                call_deselect(&mut tree[tabs.selected].tab_type, &mut commands, &mut viewport);
                                tabs.selected = i.clone();
                                call_select(&mut tree[i].tab_type, &mut commands, &mut viewport);
                            }

                            if ui.button("x").clicked() {
                                // get closing tab type
                                let tab_type = &mut tree[i].tab_type;

                                // call deselect if this is the selected tab
                                if tabs.selected == i {
                                    call_deselect(tab_type, &mut commands, &mut viewport);
                                }

                                // close the tab and remove it from the stream
                                call_close(tab_type);
                                tabs.tree.remove(i);
                            }
                        });
                    }
                });
            });
        });

        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
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
                EditorTabType::Input(input) => input.ui(ui, &mut inputs, &mut key_events),
                EditorTabType::Model(model) => model.ui(ui, &mut viewport, rendered_texture_id),
                EditorTabType::Unknown => draw_unknown(ui, tab)
            };
        });
    // });
}

fn draw_unknown(ui: &mut egui::Ui, tab: &EditorTab) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Unknown file type for file {}", tab.name));
    });
}

fn call_close(tab_type: &mut EditorTabType) {
    match tab_type {
        EditorTabType::Input(editor) => editor.close(),
        EditorTabType::Model(model) => model.close(),
        EditorTabType::Image(_) => {}, // TODO
        EditorTabType::Text(_) => {},
        EditorTabType::Unknown => {}
    }
}

pub fn call_select(tab_type: &mut EditorTabType, commands: &mut Commands, viewport: &mut ResMut<ViewportContainer>) {
    match tab_type {
        EditorTabType::Input(editor) => editor.select(),
        EditorTabType::Model(model) => model.select(commands, viewport),
        EditorTabType::Image(_) => {}, // TODO
        EditorTabType::Text(_) => {},
        EditorTabType::Unknown => {}
    }
}

fn call_deselect(tab_type: &mut EditorTabType, commands: &mut Commands, viewport: &mut ResMut<ViewportContainer>) {
    match tab_type {
        EditorTabType::Input(editor) => editor.deselect(),
        EditorTabType::Model(model) => model.deselect(commands, viewport),
        EditorTabType::Image(_) => {}, // TODO
        EditorTabType::Text(_) => {},
        EditorTabType::Unknown => {}
    }
}
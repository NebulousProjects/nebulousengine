use std::{path::PathBuf};

use bevy::prelude::{Resource, Image, Assets, Res};
use bevy_egui::EguiContexts;
use egui::ScrollArea;
use egui_dock::{Tree, DockArea, Style};

use crate::text_editor::*;

use self::{image_viewer::ImageRenderer, input_editor::InputEditor};

pub mod text_editor;
pub mod image_viewer;
pub mod input_editor;

#[derive(Resource)]
pub struct EditorTabs<'a> {
    pub tree: Vec<EditorTab>,
    pub selected: Option<&'a mut EditorTab>
}

impl EditorTabs<'_> {
    pub fn new() -> Self {
        Self { tree: Vec::new(), selected: None }
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
    images: Res<Assets<Image>>
) {
    egui::TopBottomPanel::top("0").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            // create a horizontal scroll area for all of the buttons
            ScrollArea::horizontal().show(ui, |ui| {
                for i in 0 .. tabs.tree.len() {
                    // if i exceedes the trees limit, break
                    if i >= tabs.tree.len() { break; }

                    // horizontally stack the tabs button and its remove button
                    ui.horizontal(|ui| {
                        let tab = &tabs.tree[i];
                        if ui.button(&tab.name).clicked() {
                            println!("TODO change tab");
                        }
                        if ui.button("x").clicked() {
                            tabs.tree.remove(i);
                        }
                    });
                }
            });
        });
    });

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        // let tab = tabs.selected;
        if tabs.selected.is_none() { return; }
        let tab = tabs.selected.as_mut().unwrap();

        let tab_type = &mut tab.tab_type;
        match tab_type {
            EditorTabType::Text(text) => text.ui(ui, &tab.path),
            EditorTabType::Image(image) => image.ui(ui, &ui.max_rect()),
            EditorTabType::Input(input) => input.ui(ui, &tab.path),
            EditorTabType::Unknown => draw_unknown(ui, tab)
        };
        // DockArea::new(&mut tabs.tree)
        //     .style(Style::from_egui(ui.style().as_ref()))
        //     .show_inside(ui, &mut TabViewer { 
        //         info: TabInfo { images: &images }
        //     });
    });
}

fn draw_unknown(ui: &mut egui::Ui, tab: &EditorTab) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Unknown file type for file {}", tab.name));
    });
}

// pub struct TabInfo<'a> {
//     images: &'a Res<'a, Assets<Image>>,
// }

// struct TabViewer<'a> {
//     info: TabInfo<'a>
// }

// impl egui_dock::TabViewer for TabViewer<'_> {
//     type Tab = EditorTab;

//     fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
//         let tab_type = &mut tab.tab_type;
//         match tab_type {
//             EditorTabType::Text(text) => text.ui(ui, &tab.path),
//             EditorTabType::Image(image) => image.ui(ui, &ui.max_rect(), &self.info),
//             EditorTabType::Input(input) => input.ui(ui, &tab.path),
//             EditorTabType::Unknown => draw_unknown(ui, tab)
//         };
//     }

//     fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
//         (&*tab.name).into()
//     }
// }
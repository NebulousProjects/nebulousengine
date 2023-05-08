use std::{path::PathBuf};

use bevy::prelude::{Resource, Image, Assets, Res};
use bevy_egui::EguiContexts;
use egui_dock::{Tree, DockArea, Style};

use crate::text_editor::*;

use self::{image_viewer::ImageRenderer, input_editor::InputEditor};

pub mod text_editor;
pub mod image_viewer;
pub mod input_editor;

#[derive(Resource)]
pub struct EditorTabs {
    pub tree: Tree<EditorTab>
}

impl EditorTabs {
    pub fn new() -> Self {
        Self { tree: Tree::new(Vec::new()) }
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
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        DockArea::new(&mut tabs.tree)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut TabViewer { 
                info: TabInfo { images: &images }
            });
    });
}

pub struct TabInfo<'a> {
    images: &'a Res<'a, Assets<Image>>,
}

struct TabViewer<'a> {
    info: TabInfo<'a>
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EditorTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let tab_type = &mut tab.tab_type;
        match tab_type {
            EditorTabType::Text(text) => text.ui(ui, &tab.path),
            EditorTabType::Image(image) => image.ui(ui, &ui.max_rect(), &self.info),
            EditorTabType::Input(input) => input.ui(ui, &tab.path),
            EditorTabType::Unknown => draw_unknown(ui, tab)
        };
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab.name).into()
    }
}

fn draw_unknown(ui: &mut egui::Ui, tab: &mut EditorTab) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Unknown file type for file {}", tab.name));
    });
}
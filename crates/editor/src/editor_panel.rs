use std::{path::PathBuf};

use bevy::prelude::{Resource, ResMut};
use bevy_egui::EguiContexts;
use egui_dock::{Tree, DockArea, Style};
use nebulousengine_utils::ViewportContainer;

use crate::text_editor::*;

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
    Unknown
}

pub fn render_editor(mut contexts: EguiContexts, viewport: ResMut<ViewportContainer>, tabs: &mut EditorTabs) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        DockArea::new(&mut tabs.tree)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut TabViewer {});
    });
}

struct TabViewer;

impl egui_dock::TabViewer for TabViewer {
    type Tab = EditorTab;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let tab_type = &mut tab.tab_type;
        match tab_type {
            EditorTabType::Text(text) => text.ui(ui),
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
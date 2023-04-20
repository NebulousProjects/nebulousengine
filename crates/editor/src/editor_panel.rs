use bevy::{prelude::*, render::render_resource::Extent3d};
use bevy_egui::EguiContexts;
use egui::*;
use nebulousengine_utils::ViewportContainer;

#[derive(Resource)]
pub struct EditorTabs {
    tree: Vec<String>
}

impl EditorTabs {
    pub fn new() -> Self {
        let mut tree = Vec::new();
        tree.push("tab1".to_string());
        tree.push("tab2".to_string());

        Self { tree }
    }
}

pub fn render_editor(mut contexts: EguiContexts, viewport: ResMut<ViewportContainer>, rendered_texture_id: Local<egui::TextureId>, tabs: &mut EditorTabs) {
    
    egui::TopBottomPanel::top("tab_buttons_container").show(contexts.ctx_mut(), |ui| {
        render_editor_tabs(ui, tabs);
    });
    
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_editor_main(ui, viewport, rendered_texture_id);
    });
}

// function that renders tabs in top menu
fn render_editor_tabs(ui: &mut Ui, tabs: &mut EditorTabs) {
    // create a menu bar for the tabs
    ui.horizontal(|ui| {
        for element in tabs.tree.iter() {
            if ui.button(element).clicked() {
                println!("Element clicked {}", element);
            }
        }
    });
}

fn render_editor_main(ui: &mut Ui, mut viewport: ResMut<ViewportContainer>, rendered_texture_id: Local<egui::TextureId>/*, tabs: &mut EditorTabs */) {
    let rect = ui.max_rect();
    viewport.size = Extent3d { width: rect.width() as u32, height: rect.height() as u32, depth_or_array_layers: 1 };
    ui.add(egui::widgets::Image::new(
        *rendered_texture_id,
        [ rect.width(), rect.height() ]
    ));
}
// Example how to insert render image
// ui.add(egui::widgets::Image::new(
//     *rendered_texture_id,
//     [512.0, 512.0]
// ));
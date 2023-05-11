use std::path::PathBuf;

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_egui::*;
use editor_panel::image_viewer::ImageRenderer;
use editor_panel::input_editor::InputEditor;
use editor_panel::text_editor::TextContainer;
use nebulousengine_input::InputContainer;
use nebulousengine_utils::{ViewportContainer};
use self::files_editor_panel::render_files;
use self::editor_panel::*;

pub mod files_editor_panel;
pub mod editor_panel;

pub struct EditorOpenFileEvent {
    path: PathBuf
}
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ViewportContainer>()
            .add_event::<EditorOpenFileEvent>()
            .insert_resource(EditorTabs::new())
            .add_plugin(EguiPlugin)
            .add_system(render_ui)
            .add_system(load_tab);
    }
}

fn load_tab(
    mut contexts: EguiContexts,
    asset_server: Res<AssetServer>,
    mut tabs: ResMut<EditorTabs>,
    mut read_open_events: EventReader<EditorOpenFileEvent>
) {
    read_open_events.iter().for_each(|event| {
        // if any tab already exists with this path, cancel
        if tabs.tree.iter().any(|tab| { tab.path == event.path }) { return; }

        // unpack event
        let path = &event.path;

        // get editor type
        let editor_type = get_tab_type(&mut contexts, &asset_server, path).unwrap_or_else(|| {
            // attempt to load the file as text, other, default ot unknown
            let input_str = std::fs::read_to_string(path.clone());
            match input_str {
                Ok(str) => EditorTabType::Text(TextContainer { text: str }),
                Err(_) => EditorTabType::Unknown
            }
        });
        
        // add tab
        let tab = EditorTab {
            path: path.clone(),
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            tab_type: editor_type
        };
        tabs.tree.push(tab);

        // call select if necessary
        if tabs.tree.len() == 1 {
            call_select(&mut tabs.tree[0].tab_type);
        }
    });
}

fn get_tab_type(contexts: &mut EguiContexts, asset_server: &AssetServer, path: &PathBuf) -> Option<EditorTabType> {
    // get extension
    let extension = path.extension();
    if extension.is_none() { return None; }
    let extension = extension.unwrap().to_str();
    if extension.is_none() { return None; }
    let extension = extension.unwrap();

    // get bevy path
    let mut bevy_path = path.to_str().expect("Could not convert path for image load");
    if bevy_path.starts_with("./assets") {
        bevy_path = &bevy_path[9 .. bevy_path.len()];
    }

    warn!("Loading path {}", path.clone().as_os_str().to_str().unwrap());

    // match extension to load method
    match extension {
        "png" => Some(EditorTabType::Image(ImageRenderer::new(asset_server, contexts, bevy_path))),
        "input" => Some(EditorTabType::Input(InputEditor::new(asset_server, bevy_path))),
        _ => None
    }
}

fn render_ui(
    mut contexts: EguiContexts, 
    // viewport: ResMut<ViewportContainer>, 
    tabs: ResMut<EditorTabs>,
    mut events: EventWriter<EditorOpenFileEvent>,

    images: Res<Assets<Image>>,
    inputs: ResMut<Assets<InputContainer>>,
    key_events: EventReader<KeyboardInput>
) {
    // create side panel for files and menu buttons
    egui::SidePanel::left("master_left").resizable(true).min_width(100.0).show(contexts.ctx_mut(), |ui| { 
        // add menu buttons
        egui::TopBottomPanel::top("master_left_top_buttons").show_inside(ui, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("Open Project...").clicked() { println!("TODO open project") }
                    if ui.button("New Project...").clicked() { println!("TODO new project") }
                });
            });
        });

        // render files
        render_files(ui, &mut events)
    });

    // render editor
    render_editor(
        contexts, /*viewport, rendered_texture_id,*/ 
        tabs.into_inner(), images, 
        inputs, key_events
    );
}
// Example how to insert render image
// ui.add(egui::widgets::Image::new(
//     *rendered_texture_id,
//     [512.0, 512.0]
// ));
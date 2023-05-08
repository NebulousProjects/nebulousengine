use std::path::PathBuf;

use bevy::prelude::*;
use bevy_egui::*;
use editor_panel::image_viewer::ImageRenderer;
use editor_panel::input_editor::InputEditor;
use editor_panel::text_editor::TextContainer;
use nebulousengine_utils::{ViewportContainer, load_file_to_json};
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
        tabs.tree.push(EditorTab {
            path: path.clone(),
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            tab_type: editor_type
        });
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
        "png" => {
            let image: Handle<Image> = asset_server.load(bevy_path);
            let image_id = contexts.add_image(image.clone());
            Some(EditorTabType::Image(
                ImageRenderer {
                    handle: image,
                    texture: image_id,
                    texture_size: None
                }
            ))
        },
        "input" => {
            let json = load_file_to_json(bevy_path);
            if json.is_ok() {
                Some(EditorTabType::Input(
                    InputEditor {
                        json: json.unwrap()
                    }
                ))
            } else {
                warn!("Could not load input file json with error: {}", json.err().unwrap());
                None
            }
        },
        _ => None
    }
}

fn render_ui(
    mut contexts: EguiContexts, 
    // viewport: ResMut<ViewportContainer>, 
    tabs: ResMut<EditorTabs>,
    mut events: EventWriter<EditorOpenFileEvent>
) {
    // make sure we have an image handle
    // if viewport.image_handle.is_none() { return }

    // *rendered_texture_id = contexts.add_image(viewport.image_handle.clone().expect("why"));

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
        tabs.into_inner(),
    );
}
// Example how to insert render image
// ui.add(egui::widgets::Image::new(
//     *rendered_texture_id,
//     [512.0, 512.0]
// ));
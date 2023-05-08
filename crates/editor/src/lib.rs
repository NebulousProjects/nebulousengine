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
            // .add_plugin(NonEditorPlugin) // TODO: remove, this is just to load index.json
            // .add_system(setup_viewport)
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
        let path = &event.path;

        let editor_type = get_tab_type(&mut contexts, &asset_server, path).unwrap_or_else(|| {
            // attempt to load the file as text, other, default ot unknown
            let input_str = std::fs::read_to_string(path.clone());
            match input_str {
                Ok(str) => EditorTabType::Text(TextContainer { text: str }),
                Err(_) => EditorTabType::Unknown
            }
        });
        
        // add tab
        tabs.tree.push_to_focused_leaf(EditorTab {
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
    mut events: EventWriter<EditorOpenFileEvent>,
    images: Res<Assets<Image>>
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
        images
    );
}

/*
fn setup_viewport(
    mut images: ResMut<Assets<Image>>,
    mut viewport: ResMut<ViewportContainer>,
    // window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut cameras: Query<&mut Camera, With<MainCamera>>,
    mut last_size: Local<Extent3d>
) {
    let size = viewport.size;
    if *last_size == size { return; }
    *last_size = size;
    println!("Updating render image");

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    // create and set image handles
    let image_handle = images.add(image);
    viewport.image_handle = Some(image_handle.clone());

    // set render target
    if cameras.is_empty() {
        warn!("No cameras marked main camera!");
    } else {
        let mut cam = cameras.single_mut();
        cam.target = RenderTarget::Image(viewport.image_handle.clone().expect("hi"));
        cam.viewport = Some(
            Viewport {
                physical_size: UVec2 { x: size.width, y: size.height },
                physical_position: UVec2 { x: 0, y: 0 },
                depth: 0.0..1.0
            }
        )
    }
}
*/
// Example how to insert render image
// ui.add(egui::widgets::Image::new(
//     *rendered_texture_id,
//     [512.0, 512.0]
// ));
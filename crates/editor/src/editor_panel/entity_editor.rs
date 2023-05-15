use bevy::{prelude::*, render::render_resource::Extent3d};
use egui::*;
use json::{JsonValue, array};
use nebulousengine_entities::EntityContainer;
use nebulousengine_utils::{ViewportContainer, load_file_to_json, NoCameraSpawn};

use crate::{helpers::{self, *}, EditorCamera};

pub struct EntityEditor {
    handle: Handle<EntityContainer>,
    json: JsonValue,
    path: String,
    model: Option<Entity>,
    light: Option<Entity>,
    camera: Option<Entity>,
}

impl EntityEditor {
    pub fn new(asset_server: &AssetServer, path: &str) -> Self {
        let json = load_file_to_json(path);
        if json.is_err() {
            panic!("Json parse failed with error: {}", json.clone().err().unwrap()); 
        }

        Self {
            handle: asset_server.load(path), json: json.unwrap(),
            path: path.clone().to_string(),
            model: None, camera: None, light: None
        }
    }

    pub fn select(&mut self, commands: &mut Commands, viewport: &mut ResMut<ViewportContainer>) {
        // setup basic scene
        let handle = self.handle.clone();
        self.camera = Some(commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }).insert(EditorCamera).insert(NoCameraSpawn).id());
        self.model = Some(commands.spawn(handle).id());
        self.light = Some(commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 8000.0,
                ..Default::default()
            },
            ..Default::default()
        }).id());

        // enable viewport
        viewport.force_update = true;
    }

    pub fn deselect(&mut self, commands: &mut Commands, viewport: &mut ResMut<ViewportContainer>) {
        commands.entity(self.model.unwrap()).despawn_recursive();
        commands.entity(self.camera.unwrap()).despawn_recursive();
        commands.entity(self.light.unwrap()).despawn_recursive();
        self.model = None;
        self.light = None;
        self.camera = None;

        // disable viewport
        viewport.enabled = false;
    }

    pub fn close(&mut self) {}

    pub fn ui(
        &mut self, ui: &mut egui::Ui,
        viewport: &mut ResMut<ViewportContainer>,
        rendered_texture_id: Local<egui::TextureId>
    ) {
        let mut is_dirty = false;

        // make sure we have a valid transform
        if !self.json.has_key("transform") {
            let mut transform = JsonValue::new_object();
            let _ = transform.insert("position", array![0.0, 0.0, 0.0]);
            let _ = transform.insert("rotation", array![0.0, 0.0, 0.0]);
            let _ = transform.insert("scale", array![1.0, 1.0, 1.0]);
            let _ = self.json.insert("transform", transform);
            is_dirty = true; 
        }

        // create a scroll area in the side panel
        egui::SidePanel::right("components").resizable(true).min_width(200.0).show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                // add a transform editor for the base entity
                let transform_json = &mut self.json["transform"];
                is_dirty = is_dirty || ui_transform(ui, transform_json);

                // then a components editor
                ui.separator();
                is_dirty = is_dirty || ui_components(ui, &mut self.json["components"]);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // mark viewport enabled
            viewport.enabled = true;
    
            // get ui rectangle
            let rect = ui.max_rect();
    
            // update viewport rect
            viewport.size = Extent3d {
                width: rect.width() as u32,
                height: rect.height() as u32,
                ..Default::default()
            };
    
            // add image to ui
            ui.add(egui::Image::new(
                *rendered_texture_id,
                [rect.width(), rect.height()]
            ));
        });

        // if the json is dirty, save it
        if is_dirty {
            let result = std::fs::write(
                format!("./assets/{}", self.path),
                self.json.to_string()
            );
    
            // if result save returns an error, report that error
            if result.is_err() {
                error!("Input saved with error: {}", result.err().unwrap());
            }
        }
    }
}

fn ui_transform(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    let mut is_dirty = false;

    // make sure position, rotation, and scale exist in json
    if !json.has_key("position") { let _ = json.insert("position", array![ 0.0, 0.0, 0.0 ]); }
    if !json.has_key("rotation") { let _ = json.insert("rotation", array![ 0.0, 0.0, 0.0 ]); }
    if !json.has_key("scale") { let _ = json.insert("scale", array![ 1.0, 1.0, 1.0 ]); }

    // position editor made up of 3 editor_f32's
    ui.label("Position");
    let position = &mut json["position"];
    ui.horizontal(|ui| {
        is_dirty = is_dirty || helpers::edit_f32(ui, position, 0);
        is_dirty = is_dirty || helpers::edit_f32(ui, position, 1);
        is_dirty = is_dirty || helpers::edit_f32(ui, position, 2);
    });

    // same for rotation
    ui.label("Rotation");
    let rotation = &mut json["rotation"];
    ui.horizontal(|ui| {
        is_dirty = is_dirty || helpers::edit_f32(ui, rotation, 0);
        is_dirty = is_dirty || helpers::edit_f32(ui, rotation, 1);
        is_dirty = is_dirty || helpers::edit_f32(ui, rotation, 2);
    });

    // same for scale
    ui.label("Scale");
    let scale = &mut json["scale"];
    ui.horizontal(|ui| {
        is_dirty = is_dirty || helpers::edit_f32(ui, scale, 0);
        is_dirty = is_dirty || helpers::edit_f32(ui, scale, 1);
        is_dirty = is_dirty || helpers::edit_f32(ui, scale, 2);
    });

    is_dirty
}

fn ui_components(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    let mut is_dirty = false;

    // label and add button
    ui.horizontal(|ui| {
        ui.label("Components:");
        if ui.button("Add Component").clicked() {
            println!("TODO add component")
        }
    });

    // loop through all components and draw each
    for i in 0 .. json.len() {
        let component_json = &mut json[i];
        is_dirty = is_dirty || ui_component(ui, component_json);
    }

    is_dirty
}

fn ui_component(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    // get type string
    let mut is_dirty = false;
    let clone = json.clone();
    let type_str = clone["type"].as_str().unwrap();
    
    // create vertical collapsable
    ui.collapsing(type_str, |ui| {
        ui.vertical(|ui| {
            // match type to editor
            match type_str {
                "model" => {
                    ui.horizontal(|ui| {
                        ui.label("Path:");
                        is_dirty = is_dirty || edit_path(ui, json, "model");
                    });
                }
                "camera" => {
                    // enum dropdowns
                    is_dirty = is_dirty 
                        || edit_enum_dropdown(ui, json, "projection", &["perspective", "orthographic"], "perspective")
                        || edit_enum_dropdown(
                            ui, json, "tonemapping", 
                            &["none", "reinhard", "reinhard_lumninance", "aces_fitted", "agx", "somewhat_boring_display_transform", "tony_mc_mapface", "blender_filmic"], 
                            "reinhard_luminance"
                        );

                    // booleans
                    is_dirty = is_dirty || edit_bool(ui, json, "hdr", false)
                        || edit_bool(ui, json, "msaa_writeback", true)
                        || edit_bool(ui, json, "deband_dither", true)
                        || edit_bool(ui, json, "show_ui", true)
                        || edit_bool(ui, json, "main", false);

                    ui.separator();

                    // color grading
                    if !json.has_key("color_grading") { let _ = json.insert("color_grading", JsonValue::new_object()); }
                    is_dirty = is_dirty || ui_color_grading(ui, &mut json["color_grading"]);

                }
                _ => { error!("Unknown type string {}", type_str); }
            }
        });
    });
    is_dirty
}

fn ui_color_grading(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    ui.label("Color Grading:");
    edit_slider(ui, json, "exposure", 0.0, 1.0, 0.0) ||
    edit_slider(ui, json, "gamma", 0.0, 1.0, 1.0) ||
    edit_slider(ui, json, "pre_saturation", 0.0, 1.0, 1.0) ||
    edit_slider(ui, json, "post_saturation", 0.0, 1.0, 1.0)
}

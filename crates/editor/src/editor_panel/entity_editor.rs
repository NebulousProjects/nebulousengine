use bevy::{prelude::*, render::render_resource::Extent3d};
use egui::*;
use json::{JsonValue, array};
use nebulousengine_entities::EntityContainer;
use nebulousengine_utils::{ViewportContainer, MainCamera, load_file_to_json};

use crate::helpers;

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
        }).insert(MainCamera).id());
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

        // create a scroll area in the side panel
        egui::SidePanel::right("components").resizable(true).min_width(200.0).show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                // add a transform editor for the base entity
                let transform_json = &mut self.json["transform"];
                is_dirty = is_dirty || ui_transform(ui, transform_json);

                // then a components editor

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

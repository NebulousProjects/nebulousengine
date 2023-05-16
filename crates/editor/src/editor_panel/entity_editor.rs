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

                // then children
                ui.separator();
                ui_children(ui, &mut self.json["children"]);
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

fn ui_children(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    let mut is_dirty = false;

    ui.horizontal(|ui| {
        ui.label("Children: ");
        if ui.button("Add Children").clicked() {
            let mut new_json = JsonValue::new_object();
            let _ = new_json.insert("transform", JsonValue::new_object());
            let _ = new_json.insert("components", JsonValue::new_array());
            let _ = new_json.insert("children", JsonValue::new_array());
            let _ = json.push(new_json);
            is_dirty = true;
        }
    });

    let mut opt: Option<usize> = None;
    for i in 0 .. json.len() {
        ui.collapsing(format!("{}", i), |ui| {
            ui.vertical(|ui| {
                let json = &mut json[i];

                // remove button
                if ui.button("Remove").clicked() {
                    opt = Some(i);
                }

                // add a transform editor for the base entity
                let transform_json = &mut json["transform"];
                is_dirty = is_dirty || ui_transform(ui, transform_json);

                // then a components editor
                ui.separator();
                is_dirty = is_dirty || ui_components(ui, &mut json["components"]);

                // then children
                ui.separator();
                ui_children(ui, &mut json["children"]);
            });
        });
    }

    // remove if option is something
    if opt.is_some() {
        json.array_remove(opt.unwrap());
        is_dirty = true;
    }

    is_dirty
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
        is_dirty = is_dirty || helpers::edit_f32(ui, position, 0, 0.0);
        is_dirty = is_dirty || helpers::edit_f32(ui, position, 1, 0.0);
        is_dirty = is_dirty || helpers::edit_f32(ui, position, 2, 0.0);
    });

    // same for rotation
    ui.label("Rotation");
    let rotation = &mut json["rotation"];
    ui.horizontal(|ui| {
        is_dirty = is_dirty || helpers::edit_f32(ui, rotation, 0, 0.0);
        is_dirty = is_dirty || helpers::edit_f32(ui, rotation, 1, 0.0);
        is_dirty = is_dirty || helpers::edit_f32(ui, rotation, 2, 0.0);
    });

    // same for scale
    ui.label("Scale");
    let scale = &mut json["scale"];
    ui.horizontal(|ui| {
        is_dirty = is_dirty || helpers::edit_f32(ui, scale, 0, 1.0);
        is_dirty = is_dirty || helpers::edit_f32(ui, scale, 1, 1.0);
        is_dirty = is_dirty || helpers::edit_f32(ui, scale, 2, 1.0);
    });

    is_dirty
}

fn ui_components(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    let mut is_dirty = false;

    // label and add button
    ui.horizontal(|ui| {
        ui.label("Components:");
        egui::ComboBox::from_id_source(-1)
            .selected_text("Add Component")
            .show_ui(ui, |ui| {
                is_dirty = is_dirty || 
                    add_component_selectable(ui, json, "model") || 
                    add_component_selectable(ui, json, "camera") || 
                    add_component_selectable(ui, json, "directional_light") || 
                    add_component_selectable(ui, json, "point_light") || 
                    add_component_selectable(ui, json, "shape") || 
                    add_component_selectable(ui, json, "elasticity") || 
                    add_component_selectable(ui, json, "collider") ||
                    add_component_selectable(ui, json, "rigidbody") || 
                    add_component_selectable(ui, json, "character_controller");
            });
    });

    // loop through all components and draw each
    let mut opt: Option<usize> = None;
    for i in 0 .. json.len() {
        let component_json = &mut json[i];
        ui.collapsing(format!("{}: {}", i, component_json.clone()["type"].as_str().unwrap()), |ui| {
            ui.vertical(|ui| {
                if ui.button("Remove").clicked() {
                    opt = Some(i);
                }
                is_dirty = is_dirty || ui_component(ui, component_json);
            });
        });
    }

    if opt.is_some() {
        json.array_remove(opt.unwrap());
        is_dirty = true;
    }

    is_dirty
}

fn add_component_selectable(ui: &mut egui::Ui, json: &mut JsonValue, type_str: &str) -> bool {
    if ui.selectable_label(false, type_str).clicked() {
        let mut new_json = JsonValue::new_object();
        let _ = new_json.insert("type", type_str);
        let _ = json.push(new_json);
        true
    } else { false }
}

fn ui_component(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    // get type string
    let mut is_dirty = false;
    let type_str = json["type"].as_str().unwrap();
    
    // create vertical collapsable
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
        "point_light" => {
            is_dirty = is_dirty || edit_color(ui, json, "color")
                || edit_f32_at_str(ui, json, "intensity", 800.0)
                || edit_f32_at_str(ui, json, "range", 20.0)
                || edit_f32_at_str(ui, json, "radius", 1.0)
                || edit_bool(ui, json, "shadows_enabled", true)
                || edit_f32_at_str(ui, json, "shadow_depth_bias", 0.02)
                || edit_f32_at_str(ui, json, "shadow_normal_bias", 0.6);
        }
        "directional_light" => {
            is_dirty = is_dirty || edit_color(ui, json, "color")
                || edit_f32_at_str(ui, json, "intensity", 800.0)
                || edit_bool(ui, json, "shadows_enabled", true)
                || edit_f32_at_str(ui, json, "shadow_depth_bias", 0.02)
                || edit_f32_at_str(ui, json, "shadow_normal_bias", 0.6);
        }
        "elasticity" => {
            is_dirty = is_dirty || edit_f32_at_str(ui, json, "elasticity", 0.0);
        }
        "rigidbody" => {
            is_dirty = is_dirty ||
                edit_enum_dropdown(ui, json, "physics_type", &["dynamic", "fixed", "kinematic_position", "kinematic_velocity"], "dynamic")
        }
        "character_controller" => {
            is_dirty = is_dirty ||
                edit_vec3(ui, json, "translation", 0.0, 0.0, 0.0) ||
                edit_vec3(ui, json, "up", 0.0, 1.0, 0.0) ||
                edit_f32_at_str(ui, json, "mass", 1.0) ||
                edit_f32_at_str(ui, json, "offset", 0.01) ||
                edit_f32_at_str(ui, json, "snap_to_ground", 0.2) ||
                edit_f32_at_str(ui, json, "max_climb_angle", 45.0) ||
                edit_f32_at_str(ui, json, "min_slide_angle", 30.0) ||
                edit_bool(ui, json, "slide_enabled", false) ||
                edit_bool(ui, json, "move_dynamic_bodies", true);

            ui.separator();

            ui.label("Autostep:");
            if !json.has_key("autostep") { let _ = json.insert("autostep", JsonValue::new_object()); }
            let autostep = &mut json["autostep"];
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, autostep, "max_height", 0.25) ||
                edit_f32_at_str(ui, autostep, "min_width", 0.1) ||
                edit_bool(ui, autostep, "include_dynamic_bodies", true);
        }
        "collider" => {
            is_dirty = is_dirty || 
                edit_enum_dropdown(ui, json, "shape", &[
                    "sphere", "cylinder", "rounded_cylinder", "cone",
                    "rounded_cone", "capsule", "cube", "rounded_cube",
                    "triangle", "rounded_triangle"
                ], "sphere") ||
                ui_collision_shape(ui, json) ||
                edit_f32_at_str(ui, json, "mass", 1.0) ||
                edit_f32_at_str(ui, json, "friction", 1000.0) ||
                edit_bool(ui, json, "is_sensor", false);
        }
        "shape" => {
            is_dirty = is_dirty ||
                edit_color(ui, json, "color") ||
                edit_enum_dropdown(ui, json, "shape", &[
                    "box", "capsule", "circle", "cube",
                    "cylinder", "ico_sphere", "plane",
                    "quad", "polygon", "torus", "sphere"
                ], "ico_sphere") ||
                ui_mesh_shape(ui, json);
        }
        _ => { error!("Unknown type string {}", type_str); }
    }

    is_dirty
}

fn ui_mesh_shape(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    let mut is_dirty = false;

    match json["shape"].as_str().unwrap() {
        "box" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "width", 1.0) ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "depth", 1.0);
        }
        "capsule" => {
            is_dirty = is_dirty ||
                edit_enum_dropdown(ui, json, "uv_profile", &[
                    "aspect", "uniform", "fixed"
                ], "fixed") ||
                edit_f32_at_str(ui, json, "radius", 0.5) ||
                edit_f32_at_str(ui, json, "depth", 1.0) ||
                edit_usize(ui, json, "rings", 0) ||
                edit_usize(ui, json, "latitudes", 16) ||
                edit_usize(ui, json, "longitudes", 32);
        }
        "circle" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "raadius", 0.5) ||
                edit_usize(ui, json, "vertices", 64);
        }
        "cube" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "size", 1.0);
        }
        "cylinder" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "radius", 0.5) ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_u32(ui, json, "resolution", 16) ||
                edit_u32(ui, json, "segments", 1);
        }
        "ico_sphere" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "radius", 1.0) ||
                edit_usize(ui, json, "subdivisions", 5);
        }
        "plane" => {
            is_dirty = is_dirty || edit_f32_at_str(ui, json, "size", 1.0);
        }
        "quad" => {
            is_dirty = is_dirty ||
                edit_vec2(ui, json, "size", 1.0, 1.0) ||
                edit_bool(ui, json, "flip", false);
        }
        "polygon" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "radius", 1.0) ||
                edit_usize(ui, json, "sides", 3);
        }
        "torus" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "radius", 1.0) ||
                edit_f32_at_str(ui, json, "ring_radius", 0.5) ||
                edit_usize(ui, json, "subdivisions_segments", 32) ||
                edit_usize(ui, json, "subdivisions_sides", 24);
        }
        "sphere" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "radius", 1.0) ||
                edit_usize(ui, json, "sectors", 36) ||
                edit_usize(ui, json, "stacks", 18);
        }
        _ => { error!("Unknown mesh shape"); }
    }

    is_dirty
}

fn ui_collision_shape(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    let mut is_dirty = false;

    match json["shape"].as_str().unwrap() {
        "sphere" => { 
            is_dirty = is_dirty || edit_f32_at_str(ui, json, "radius", 1.0); 
        }
        "cylinder" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "radius", 1.0);
        }
        "rounded_cylinder" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "radius", 1.0) ||
                edit_f32_at_str(ui, json, "border_radius", 0.0);
        }
        "cone" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "radius", 1.0);
        }
        "rounded_cone" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "radius", 1.0) ||
                edit_f32_at_str(ui, json, "border_radius", 0.0);
        }
        "capsule" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "radius", 1.0);
        }
        "cube" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "width", 1.0) ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "depth", 1.0);
        }
        "rounded_cube" => {
            is_dirty = is_dirty ||
                edit_f32_at_str(ui, json, "width", 1.0) ||
                edit_f32_at_str(ui, json, "height", 1.0) ||
                edit_f32_at_str(ui, json, "depth", 1.0) ||
                edit_f32_at_str(ui, json, "radius", 1.0);
        }
        "triangle" => {
            is_dirty = is_dirty ||
                edit_vec3(ui, json, "point_a", 0.0, 0.0, 0.0) ||
                edit_vec3(ui, json, "point_b", 0.0, 0.0, 0.0) ||
                edit_vec3(ui, json, "point_c", 0.0, 0.0, 0.0);
        }
        "rounded_triangle" => {
            is_dirty = is_dirty ||
                edit_vec3(ui, json, "point_a", 0.0, 0.0, 0.0) ||
                edit_vec3(ui, json, "point_b", 0.0, 0.0, 0.0) ||
                edit_vec3(ui, json, "point_c", 0.0, 0.0, 0.0) ||
                edit_f32_at_str(ui, json, "radius", 1.0);
        }
        _ => { error!("Unknown collision shape"); }
    };

    is_dirty
}

fn ui_color_grading(ui: &mut egui::Ui, json: &mut JsonValue) -> bool {
    ui.label("Color Grading:");
    edit_slider(ui, json, "exposure", 0.0, 1.0, 0.0) ||
    edit_slider(ui, json, "gamma", 0.0, 1.0, 1.0) ||
    edit_slider(ui, json, "pre_saturation", 0.0, 1.0, 1.0) ||
    edit_slider(ui, json, "post_saturation", 0.0, 1.0, 1.0)
}

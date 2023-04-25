use std::ops::Add;

use bevy::{prelude::*, ecs::system::EntityCommands};
use json::JsonValue;
use nebulousengine_utils::{*, optionals::*, enums::*};

pub fn spawn_entity_from_path(
    commands: &mut Commands, 
    path: &str, 
    asset_server: &Res<AssetServer>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>
) {
    spawn_entity_from_json(commands, &load_file_to_json(path), asset_server, position_offset, rotation_offset, scale_mult);
}

pub fn spawn_entity_from_json(
    commands: &mut Commands, 
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>
) {
    // create entity
    let mut entity = commands.spawn_empty();

    // unpack json
    let components = &input_json["components"];

    // if components is an array, loop through each component
    if components.is_array() {
        for i in 0 .. components.len() {
            // add each component to the entity
            let bundle = unpack_component(&components[i], asset_server);
            if bundle.is_ok() {
                bundle.unwrap().attach(&mut entity);
            } else {
                warn!("Failed to convert json to component with input {}", input_json)
            }
        }
    }
    
    // add transform after components to override any transforms created
    let mut transform = optional_transform(input_json, "transform");
    if position_offset.is_some() { transform.translation += position_offset.unwrap() }
    if rotation_offset.is_some() { transform.rotate(rotation_offset.unwrap()) }
    if scale_mult.is_some() { transform.scale *= scale_mult.unwrap() }
    entity.insert(transform);
}

pub enum EntityBundle {
    Model(SceneBundle),
    Camera((Camera3dBundle, UiCameraConfig, Option<MainCamera>))
}

impl EntityBundle {
    fn attach(self, commands: &mut EntityCommands) {
        match self {
            Self::Model(bundle) => commands.insert(bundle),
            Self::Camera((camera, ui_config, main_camera)) => {
                // insert not optional camera components
                commands.insert(camera).insert(ui_config);

                // add main camera if able
                if main_camera.is_some() { commands.insert(main_camera.unwrap()); }

                // return the commands to match others
                commands
            } 
        };
    }
}

fn unpack_component(input_json: &JsonValue, asset_server: &Res<AssetServer>) -> Result<EntityBundle, String> {
    // unpack json
    let type_str = input_json["type"].as_str();

    // make sure unpacked info is correct
    if type_str.is_none() {
        return Err(format!("Could not grab type from component"));
    }
    let type_str = type_str.unwrap();

    // match the component and add it to the entity
    Ok(match type_str {
        "model" => {
            EntityBundle::Model(
                SceneBundle {
                    scene: asset_server.load(format!("{}#Scene0", optional_string(input_json, "model")).as_str()),
                    visibility: visibility(optional_string(input_json, "visibility")),
                    ..Default::default()
                }
            )
        },
        "camera" => {
            EntityBundle::Camera((
                Camera3dBundle {
                    camera: Camera {
                        viewport: optional_viewport(input_json, "viewport"),
                        order: optional_isize(input_json, "order", 0),
                        is_active: optional_bool(input_json, "active", true),
                        hdr: optional_bool(input_json, "hdr", false), // WARN EXPERIMENTAL
                        msaa_writeback: optional_bool(input_json, "msaa_writeback", true), // WARN EXPERIMENTAL
                        ..Default::default()
                    },
                    projection: projection(optional_string(input_json, "projection")),
                    tonemapping: tonemapping(optional_string(input_json, "tonemapping")),
                    dither: optional_deband_dither(input_json, "dither"),
                    color_grading: optional_color_grading(input_json, "color_grading"),
                    ..Default::default()
                },
                UiCameraConfig {
                    show_ui: optional_bool(input_json, "show_ui", true)
                },
                if optional_bool(input_json, "main", false) { Some(MainCamera) } else { None }
            ))
        }
        _ => return Err(format!("Could not add type {}", type_str))
    })
}
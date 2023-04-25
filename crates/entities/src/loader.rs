use bevy::{prelude::*, ecs::system::EntityCommands};
use json::JsonValue;
use nebulousengine_utils::optionals::*;

use crate::components::unpack_component;

pub fn add_children(
    commands: &mut EntityCommands, 
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>,
    visible: bool
) {
    // check if we have a children json object and grab it
    if input_json.has_key("children") {
        let children = &input_json["children"];

        // make sure object is array
        if children.is_array() {
            // create a new child for each element in children array
            commands.with_children(|builder| {
                for i in 0 .. children.len() {
                    let json = &children[i];
                    let mut entity = builder.spawn_empty();
                    build_entity_from_json(
                        &mut entity, json, asset_server, 
                        meshes, materials, position_offset, 
                        rotation_offset, scale_mult, visible
                    );
                    add_children(
                        &mut entity, json, asset_server, 
                        meshes, materials, position_offset, 
                        rotation_offset, scale_mult, visible
                    );
                }
            });
        }
    }
}

pub fn build_entity_from_json(
    entity: &mut EntityCommands, 
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>,
    visible: bool
) {
    // unpack json
    let components = &input_json["components"];

    // add visibility
    entity.insert(
        if visible { Visibility::Inherited }
        else { Visibility::Hidden }
    ).insert(ComputedVisibility::default());
    
    // add transform
    let mut transform = optional_transform(input_json, "transform");
    if position_offset.is_some() { transform.translation += position_offset.unwrap() }
    if rotation_offset.is_some() { transform.rotate(rotation_offset.unwrap()) }
    if scale_mult.is_some() { transform.scale *= scale_mult.unwrap() }
    entity.insert(transform).insert(GlobalTransform::default());

    // if components is an array, loop through each component
    if components.is_array() {
        for i in 0 .. components.len() {
            // add each component to the entity
            let bundle = unpack_component(&components[i], asset_server, meshes, materials);
            if bundle.is_ok() {
                bundle.unwrap().attach(entity);
            } else {
                warn!("Failed to convert json to component with input {}", input_json)
            }
        }
    }
}

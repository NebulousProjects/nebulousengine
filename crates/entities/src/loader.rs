use bevy::{prelude::*, ecs::system::EntityCommands};
use json::JsonValue;
use nebulousengine_utils::optionals::*;

use crate::components::unpack_component;

pub fn add_children(
    commands: &mut EntityCommands, 
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
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
                        meshes, materials
                    );
                    add_children(
                        &mut entity, json, asset_server, 
                        meshes, materials
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
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    // unpack json
    let components = &input_json["components"];

    // add visibility
    entity.insert(Visibility::Inherited).insert(ComputedVisibility::default());

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
    
    // add transform
    let transform = optional_transform(input_json, "transform");
    entity.insert(transform).insert(GlobalTransform::default());
}

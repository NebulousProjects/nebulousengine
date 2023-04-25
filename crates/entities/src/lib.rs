use bevy::prelude::*;
use json::JsonValue;
use nebulousengine_utils::*;

use self::loader::*;

pub mod components;
pub mod loader;

pub fn spawn_entity_from_path(
    commands: &mut Commands, 
    path: &str, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>,
    visible: bool
) {
    spawn_entity_from_json(commands, &load_file_to_json(path), asset_server, meshes, materials, position_offset, rotation_offset, scale_mult, visible);
}

pub fn spawn_entity_from_json(
    commands: &mut Commands, 
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>,
    visible: bool
) {
    // create entity
    let mut entity = commands.spawn_empty();

    // call build functions
    build_entity_from_json(
        &mut entity, input_json, asset_server, 
        meshes, materials, position_offset, 
        rotation_offset, scale_mult, visible
    );
    add_children(
        &mut entity, input_json, asset_server, 
        meshes, materials, position_offset, 
        rotation_offset, scale_mult, visible
    );
}

use bevy::prelude::*;
use json::JsonValue;
use nebulousengine_utils::{*, optionals::*};
use bevy_rapier3d::prelude::*;

use self::loader::*;

pub mod components;
pub mod loader;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

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
    let json = load_file_to_json(path);
    if json.is_ok() {
        spawn_entity_from_json(commands, &json.unwrap(), asset_server, meshes, materials, position_offset, rotation_offset, scale_mult, visible);
    } else {
        error!("{}", json.err().unwrap())
    }
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
    
    // add despawnable unless marked persistent
    if !optional_bool(input_json, "persistent", false) {
        entity.insert(Despawnable);
    }

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

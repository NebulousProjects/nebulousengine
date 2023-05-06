use bevy::{prelude::*, ecs::system::EntityCommands, asset::{AssetLoader, LoadedAsset, HandleId}, reflect::TypeUuid};
use json::JsonValue;
use nebulousengine_utils::{*, optionals::*};
use bevy_rapier3d::prelude::*;

use self::loader::*;

pub mod components;
pub mod loader;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<EntityContainer>()
            .init_asset_loader::<EntityLoader>()
            .add_system(load_entities)
            .add_system(reload)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

#[derive(Component, TypeUuid)]
#[uuid = "135601b6-2de3-4497-8f4b-3f4841948584"]
pub struct EntityContainer {
    json: JsonValue
}

#[derive(Component)]
pub struct EntityContainerLoaded {
    path: HandleId
}

#[derive(Default)]
pub struct EntityLoader;
impl AssetLoader for EntityLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // get json from bytes
            let str = std::str::from_utf8(bytes);
            if str.is_err() { return Err(bevy::asset::Error::msg("Could not convert bytes to json in entity asset loader")) }
            let root_json = json::parse(str.unwrap());
            if root_json.is_err() { return Err(bevy::asset::Error::msg("Could not parse json in entity asset loader")) }
            let root_json = root_json.unwrap();

            // save root json
            load_context.set_default_asset(LoadedAsset::new(EntityContainer { json: root_json }));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["entity"]
    }
}

fn reload(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<EntityContainer>>,
    query: Query<(Entity, &EntityContainerLoaded), With<EntityContainerLoaded>>,
) {
    for event in &mut ev_asset {
        match event {
            AssetEvent::Modified { handle } => {
                warn!("Reloading entity containers");
                let id = handle.id();
                query.iter().filter(|(_, loaded)| {
                    id == loaded.path
                }).for_each(|(entity, _)| {
                    let mut entity_cmds = commands.entity(entity);
                    entity_cmds.remove::<EntityContainerLoaded>();
                    entity_cmds.despawn_descendants();
                });
            }
            _ => { warn!("Unhandled") }
        }
    }
}

fn load_entities(
    query: Query<(Entity, &Handle<EntityContainer>), Without<EntityContainerLoaded>>,
    assets: Res<Assets<EntityContainer>>,
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // loop through all entity containers without entity container loaded
    for (entity, handle) in query.iter() {
        // get entity container if it exists
        let container = assets.get(handle);
        if container.is_some() {
            // get path
            let path = handle.id();

            // add the entity components to entity from the json
            let mut entity_commands = commands.entity(entity);
            spawn_entity_from_json(&mut entity_commands, &container.unwrap().json, &asset_server, &mut meshes, &mut materials);
            entity_commands.insert(EntityContainerLoaded { path: path } );
        }
    }
}

pub fn spawn_entity_from_json(
    commands: &mut EntityCommands, 
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    // add despawnable unless marked persistent
    if !optional_bool(input_json, "persistent", false) {
        commands.insert(Despawnable);
        // commands.
    }

    // call build functions
    build_entity_from_json(
        commands, input_json, asset_server, 
        meshes, materials
    );
    add_children(
        commands, input_json, asset_server, 
        meshes, materials
    );
}

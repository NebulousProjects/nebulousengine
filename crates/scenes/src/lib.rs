use bevy::{prelude::*, asset::*, reflect::*};
use json::JsonValue;
use loader::load_scene_from_json;
use nebulousengine_utils::*;

mod loader;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<SceneContainer>()
            .init_asset_loader::<SceneLoader>()
            .add_system(load_scenes)
            .add_system(reload);
    }
}

#[derive(Component, TypeUuid)]
#[uuid = "0f15297a-3c68-4d20-a12d-1a2b9e46ea62"]
pub struct SceneContainer {
    json: JsonValue
}

#[derive(Component)]
pub struct SceneContainerLoaded {
    path: HandleId
}

#[derive(Default)]
pub struct SceneLoader;
impl AssetLoader for SceneLoader {
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
            load_context.set_default_asset(LoadedAsset::new(SceneContainer { json: root_json }));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["scene"]
    }
}

fn load_scenes(
    query: Query<(Entity, &Handle<SceneContainer>), Without<SceneContainerLoaded>>,
    entities: Query<Entity, With<Despawnable>>,
    assets: Res<Assets<SceneContainer>>,
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    no_cam_spawn: Query<(Entity, With<NoCameraSpawn>)>,
) {
    // loop through all scene containers without scene container loaded
    for (entity, handle) in query.iter() {
        // get scene container if it exists
        let container = assets.get(handle);
        if container.is_some() {
            // get id
            let id = handle.id();

            load_scene(&mut commands, &asset_server, &entities, &mut meshes, &mut materials, &no_cam_spawn, &container.unwrap().json);

            commands.entity(entity).insert(SceneContainerLoaded { path: id });
        }
    }
}

fn reload(
    mut commands: Commands,
    mut ev_assets: EventReader<AssetEvent<SceneContainer>>,
    query: Query<(Entity, &SceneContainerLoaded), With<SceneContainerLoaded>>,
) {
    for event in &mut ev_assets {
        match event {
            AssetEvent::Modified { handle } => {
                let id = handle.id();
                query.iter().filter(|(_, loaded)| {
                    id == loaded.path
                }).for_each(|(entity, _)| {
                    let mut entity_cmds = commands.entity(entity);
                    entity_cmds.remove::<SceneContainerLoaded>();
                });
            }
            _ => {}
        }
    }
}

fn load_scene(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entities: &Query<Entity, With<Despawnable>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    no_cam_spawn: &Query<(Entity, With<NoCameraSpawn>)>,
    json: &JsonValue
) {
    // clear old entities and UIs
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // load new scene
    load_scene_from_json(
        commands, json, 
        &asset_server, meshes, 
        materials, no_cam_spawn
    );
}
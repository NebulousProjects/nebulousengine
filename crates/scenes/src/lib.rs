use bevy::prelude::*;
use loader::load_scene_from_path;
use nebulousengine_scripting::*;
use nebulousengine_utils::*;

mod loader;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SceneInfo::default())
            .add_startup_system(start);
    }
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut wrapper: NonSendMut<ScriptEngineWrapper>
) {
    load_scene_from_path(&mut commands, "./assets/test.scene", &asset_server, &mut meshes, &mut materials, &mut wrapper)
}


#[derive(Resource)]
pub struct SceneInfo {
    next_scene: String
}

impl Default for SceneInfo {
    fn default() -> Self {
        Self { next_scene: Default::default() }
    }
}

pub fn load_scene(
    commands: &mut Commands,
    next_scene: String,
    scene_info: &ResMut<SceneInfo>,
    entities: &Query<Entity, With<Despawnable>>, // theres gotta be a better way to do this
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    wrapper: &mut NonSendMut<ScriptEngineWrapper>
) {
    // TODO add pause here

    // call stop on all scripts not marked persistent

    // remove scripts all scripts not marked persistent

    // clear old entities and UIs
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }

    // load new scene
    load_scene_from_path(commands, next_scene.as_str(), asset_server, meshes, materials, wrapper)

    // call start on all scripts

    // TODO add resume here
}
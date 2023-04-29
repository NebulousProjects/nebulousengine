use bevy::prelude::*;
use loader::load_scene_from_path;
use nebulousengine_utils::*;

mod loader;

pub struct ScenePlugin;
pub struct LoadSceneEvent { pub path: String }

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<LoadSceneEvent>()
            .add_startup_system(start)
            .add_system(load_scene_loop);
    }
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    // mut wrapper: NonSendMut<ScriptEngineWrapper>
) {
    load_scene_from_path(&mut commands, "./assets/test.scene", &asset_server, &mut meshes, &mut materials)
}

fn load_scene_loop(
    mut commands: Commands,
    mut events: EventReader<LoadSceneEvent>,
    entities: Query<Entity, With<Despawnable>>, // theres gotta be a better way to do this
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut wrapper: NonSendMut<ScriptEngineWrapper>,
    mut running_state: ResMut<RunningState>
) {
    // run for each event
    for event in events.iter() {
        let running_state = running_state.as_mut();
        
        // pause
        running_state.running = false;

        // call stop on all scripts
        // execute_functions(&mut wrapper, "stop".to_string()); TODO

        // remove scripts all scripts
        // wrapper.engine.scripts.clear();

        // clear old entities and UIs
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // load new scene
        load_scene_from_path(
            &mut commands, event.path.as_str(), 
            &asset_server, &mut meshes, 
            &mut materials
        );

        // call start on all scripts
        // execute_functions(&mut wrapper, "start".to_string()); TODO

        // resume
        running_state.running = true;
    }
}
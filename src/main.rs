use bevy::prelude::*;

use nebulousengine_scenes::*;
use nebulousengine_scripting::*;
use nebulousengine_ui::*;
use nebulousengine_utils::RunningState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         title: "Nebulous Engine Game".into(), // todo make this dynamic
        //         present_mode: PresentMode::AutoNoVsync,
                
        //         ..default()
        //     }),
        //     ..default()
        // }))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(EditorPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ScriptingPlugin)
        .add_plugin(ScenePlugin)
        .insert_resource(RunningState::default())
        // .add_startup_system(setup)
        .add_system(update)
        .run();
}


// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     asset_server: Res<AssetServer>,
//     mut wrapper: NonSendMut<ScriptEngineWrapper>
// ) {
//     // load_scene_from_path(&mut commands, "./assets/test.scene", &asset_server, &mut meshes, &mut materials, &mut wrapper);
// }

fn update(
    mut query: Query<&mut Transform, With<Handle<Scene>>>, 
    time: Res<Time>, 
    keys: Res<Input<KeyCode>>,

    mut load_scene_events: EventWriter<LoadSceneEvent>,
    mut running_state: ResMut<RunningState>
) {
    // rotate queried entities for testing
    if running_state.running {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() / 2.);
        }
    }

    // if keys pressed, trigger scene swap
    if keys.just_released(KeyCode::A) {
        load_scene_events.send(LoadSceneEvent { path: "./assets/test2.scene".to_string() });
    } else if keys.just_released(KeyCode::D) {
        load_scene_events.send(LoadSceneEvent { path: "./assets/test.scene".to_string() });
    }

    // if space pressed, toggle pause
    if keys.just_released(KeyCode::Space) {
        running_state.running = !running_state.running
    }
}
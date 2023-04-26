use bevy::prelude::*;

use nebulousengine_scenes::*;
use nebulousengine_scripting::*;
use nebulousengine_ui::*;
use nebulousengine_utils::*;
// use nebulousengine_ui::convert_uifile_to_uibundle;
// use nebulousengine_editor::EditorPlugin;

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
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Handle<Scene>>>, 
    time: Res<Time>, 
    keys: Res<Input<KeyCode>>,

    scene_info: ResMut<SceneInfo>,
    entities: Query<Entity, With<Despawnable>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wrapper: NonSendMut<ScriptEngineWrapper>
) {
    if query.is_empty() { warn!("Transform query is empty!") }

    // rotate queried entities for testing
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }

    // if keys pressed, trigger scene swap
    if keys.just_released(KeyCode::A) {
        load_scene(&mut commands, "./assets/test2.scene".to_string(), &scene_info, &entities, &asset_server, &mut meshes, &mut materials, &mut wrapper);
    }
}
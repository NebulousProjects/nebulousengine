use bevy::prelude::*;

use nebulousengine_scenes::load_scene_from_path;
use nebulousengine_scripting::*;
use nebulousengine_ui::*;
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
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut wrapper: NonSendMut<ScriptEngineWrapper>
) {
    load_scene_from_path(&mut commands, "./assets/test.scene", &asset_server, &mut meshes, &mut materials, &mut wrapper);
}

fn rotate(mut query: Query<&mut Transform, With<Handle<Scene>>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}
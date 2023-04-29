use bevy::prelude::*;

use nebulousengine_input::{*, types::{InputPressedEvent, InputReleasedEvent}};
use nebulousengine_scenes::*;
use nebulousengine_scripting::*;
use nebulousengine_ui::*;
use nebulousengine_utils::*;

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
        .add_plugin(InputPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ScriptingPlugin)
        .add_plugin(ScenePlugin)
        .insert_resource(RunningState::default())
        .add_startup_system(start)
        .add_system(update)
        .run();
}

fn start(
    // mut inputs: ResMut<Inputs>
) {
    // inputs.insert_or_update_input("test2".to_string(), InputValue { 
    //     press_threshold: 1.0, 
    //     descriptions: vec![
    //         InputDescription::Scalar { input_type: InputType::Keyboard(ScanCode(30)) }
    //     ],
    //     value: 0.0
    // });
    // inputs.insert_from_path("./assets/test.input");
}

fn update(
    mut query: Query<&mut Transform, With<Handle<Scene>>>, 
    time: Res<Time>, 
    // keys: Res<Input<ScanCode>>,

    // mut load_scene_events: EventWriter<LoadSceneEvent>,
    running_state: ResMut<RunningState>,
    mut pressed_events: EventReader<InputPressedEvent>,
    mut released_events: EventReader<InputReleasedEvent>,
    // mut inputs: ResMut<Inputs>
) {
    // rotate queried entities for testing
    if running_state.running {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() / 2.);
        }
    }

    for event in pressed_events.iter() {
        info!("Pressed event: {}", event.name);
    }
    for event in released_events.iter() {
        info!("Released event: {}", event.name);
    }
}
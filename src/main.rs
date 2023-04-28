use bevy::prelude::*;

use nebulousengine_input::{*, types::{InputValue, InputRule, InputType, InputPressedEvent, InputReleasedEvent, InputDescription}};
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
    mut inputs: ResMut<Inputs>
) {
    inputs.insert_or_update_input(
        "test".to_string(),
        InputValue {
            rule: InputRule {
                press_threshold: 1.0,
                descriptions: vec![
                    InputDescription::Scalar { input_type: InputType::GamepadAxis(GamepadAxisType::LeftZ) }
                ]
            },
            value: 0.0
        }
    );
}

fn update(
    mut query: Query<&mut Transform, With<Handle<Scene>>>, 
    time: Res<Time>, 
    keys: Res<Input<KeyCode>>,

    // mut load_scene_events: EventWriter<LoadSceneEvent>,
    mut running_state: ResMut<RunningState>,
    mut pressed_events: EventReader<InputPressedEvent>,
    mut released_events: EventReader<InputReleasedEvent>,
    mut inputs: ResMut<Inputs>
) {
    // rotate queried entities for testing
    if running_state.running {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() / 2.);
        }
    }

    // if keys pressed, trigger scene swap
    // if keys.just_released(KeyCode::A) {
    //     load_scene_events.send(LoadSceneEvent { path: "./assets/test2.scene".to_string() });
    // } else if keys.just_released(KeyCode::D) {
    //     load_scene_events.send(LoadSceneEvent { path: "./assets/test.scene".to_string() });
    // }

    // for event in pressed_events.iter() {
    //     info!("Pressed event: {}", event.name);
    // }
    // for event in released_events.iter() {
    //     info!("Released event: {}", event.name);
    // }
    info!("Test current value: {}", inputs.get_value_or_default(&"test".to_string(), -1000000.0));

    // if space pressed, toggle pause
    if keys.just_released(KeyCode::Space) {
        running_state.running = !running_state.running
    }
}
use bevy::prelude::*;

use nebulousengine_editor::EditorPlugin;
use nebulousengine_input::{*, types::{InputPressedEvent, InputReleasedEvent}};
use nebulousengine_noneditor::*;
use nebulousengine_scenes::*;
use nebulousengine_ui::*;
use nebulousengine_utils::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(InputPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ScenePlugin)
        .add_system(update)
        .insert_resource(RunningState::default());

    if cfg!(feature = "editor") {
        app.add_plugin(EditorPlugin);
    } else {
        app.add_plugin(NonEditorPlugin);
    }

    app.run();
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
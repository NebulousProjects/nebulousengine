use bevy::prelude::*;
use json::JsonValue;
use nebulousengine_input::{InputContainer, types::InputPressedEvent};
use nebulousengine_scenes::LoadSceneEvent;
use nebulousengine_utils::load_file_to_json;

pub struct NonEditorPlugin;

impl Plugin for NonEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(start)
            .add_system(debug_update);
    }
}

fn start(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_scene_events: EventWriter<LoadSceneEvent>
) {
    let json = load_file_to_json("index.json");
    if json.is_ok() {
        load_index_json(&mut commands, &asset_server, &mut load_scene_events, &json.unwrap())
    } else {
        error!("Could not load index json")
    }
}

fn load_index_json(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    load_scene_events: &mut EventWriter<LoadSceneEvent>,
    json: &JsonValue
) {
    // unpack json
    let start_scene = json["start_scene"].as_str();

    // load scene if available
    if start_scene.is_some() {
        load_scene_events.send(LoadSceneEvent { path: start_scene.unwrap().to_string() })
    }
}

fn debug_update(
    mut events: EventReader<InputPressedEvent>
) {
    for event in events.iter() {
        println!("Pressed event: {}", event.name);
    }
}
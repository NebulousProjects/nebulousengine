use bevy::prelude::*;
use json::JsonValue;
use nebulousengine_scenes::LoadSceneEvent;
use nebulousengine_utils::load_file_to_json;

pub struct NonEditorPlugin;

impl Plugin for NonEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(start);
    }
}

fn start(
    mut load_scene_events: EventWriter<LoadSceneEvent>
) {
    let json = load_file_to_json("index.json");
    if json.is_ok() {
        load_index_json(&mut load_scene_events, &json.unwrap())
    } else {
        error!("Could not load index json")
    }
}

fn load_index_json(
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
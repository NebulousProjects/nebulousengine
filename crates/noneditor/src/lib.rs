use bevy::prelude::*;
use json::JsonValue;
use nebulousengine_scenes::SceneContainer;
use nebulousengine_utils::load_file_to_json;

pub struct NonEditorPlugin;

impl Plugin for NonEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(start);
    }
}

fn start(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let json = load_file_to_json("index.json");
    if json.is_ok() {
        load_index_json(&mut commands, &asset_server, &json.unwrap())
    } else {
        error!("Could not load index json")
    }
}

fn load_index_json(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    json: &JsonValue
) {
    // unpack json
    let start_scene = json["start_scene"].as_str();

    // load scene if available
    if start_scene.is_some() {
        let handle: Handle<SceneContainer> = asset_server.load(start_scene.unwrap());
        commands.spawn_empty().insert(handle);
    }
}
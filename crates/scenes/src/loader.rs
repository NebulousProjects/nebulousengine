use json::JsonValue;
use bevy::prelude::*;
use nebulousengine_entities::*;
use nebulousengine_input::InputContainer;
use nebulousengine_ui::*;
use nebulousengine_utils::*;


pub fn load_scene_from_json(
    commands: &mut Commands, 
    json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // wrapper: &mut NonSendMut<ScriptEngineWrapper>
) {
    // load entities
    if json.has_key("entities") {
        let entities = &json["entities"];
        if entities.is_array() {
            for i in 0 .. entities.len() {
                load_entity(
                    commands, &entities[i], asset_server, 
                    meshes, materials
                )
            }
        }
    }

    // load ui
    if json.has_key("uis") {
        let uis = &json["uis"];
        if uis.is_array() {
            for i in 0 .. uis.len() {
                load_ui(commands, &uis[i], asset_server)
            }
        }
    }

    // load scripts
    if json.has_key("scripts") {
        let scripts = &json["scripts"];
        if scripts.is_array() {
            // for i in 0 .. scripts.len() {
                error!("TODO scripts");
                // load_script(&scripts[i], wrapper);
            // }
        }
    }

    // load inputs
    if json.has_key("inputs") {
        let inputs = &json["inputs"];
        if inputs.is_array() {
            for i in 0 .. inputs.len() {
                let input = inputs[i].as_str();
                if input.is_some() {
                    let input_container: Handle<InputContainer> = asset_server.load(input.unwrap());
                    commands.spawn(input_container);
                }
            }
        }
    }
}

pub fn load_entity(
    commands: &mut Commands, 
    json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) {
    if json.is_object() {
        let mut entity = commands.spawn_empty();

        if json.has_key("path") {
            // get optional transform
            // let transform = optional_transform(json, "transform");

            // // get position option
            // let position = if transform.translation.length() != 0.0 {
            //     Some(transform.translation)
            // } else { None };

            // // get rotation option
            // let rotation = if transform.rotation.length() != 0.0 {
            //     Some(transform.rotation)
            // } else { None };

            // // get scale option
            // let scale = if transform.scale.length() != 1.0 {
            //     Some(transform.scale)
            // } else { None };

            // // get visible option
            // let visible = optional_bool(json, "visible", true);

            // load entity
            // spawn_entity_from_path(commands, json["path"].as_str().unwrap(), asset_server, meshes, materials, position)

            let handle: Handle<EntityContainer> = asset_server.load(json["path"].as_str().unwrap());
            entity.insert(handle);
        } else if json.has_key("components") {
            // get visible option
            // let visible = optional_bool(json, "visible", true);

            // spawn entity from json
            spawn_entity_from_json(&mut entity, json, asset_server, meshes, materials);
        } else {
            error!("Could not load entity from json {}", json);
        }
    }
}

pub fn load_ui(
    commands: &mut Commands, 
    json: &JsonValue, 
    asset_server: &Res<AssetServer>,
) {
    if json.is_string() {
        let json = load_file_to_json(json.as_str().unwrap());
        if json.is_ok() {
            add_ui_json_to_commands(&json.unwrap(), commands, asset_server);
        } else {
            error!("{}", json.err().unwrap())
        }
    } else if json.is_object() {
        add_ui_json_to_commands(json, commands, asset_server);
    } else {
        error!("Could not load ui from json {}", json);
    }
}

// pub fn load_script(
//     json: &JsonValue, 
//     wrapper: &mut NonSendMut<ScriptEngineWrapper>,
// ) {
//     if json.is_string() {
//         load_script_path(wrapper, json.as_str().unwrap().to_string())
//     } else {
//         error!("Could not load ui from json {}", json);
//     }
// }

/*
pub fn load_scene_from_path(
    commands: &mut Commands, 
    path: &str, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // wrapper: &mut NonSendMut<ScriptEngineWrapper>
) {
    let json = load_file_to_json(path);

    if json.is_ok() {
        println!("Loading scene!");
        load_scene_from_json(
            commands, &json.unwrap(), 
            asset_server, meshes, materials
        );
    } else {
        error!("{}", json.err().unwrap());
    }
}
*/
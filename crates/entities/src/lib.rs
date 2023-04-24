use bevy::{prelude::*, ecs::system::EntityCommands, gltf::Gltf};
use json::JsonValue;
use nebulousengine_utils::*;

pub fn spawn_entity_from_path(commands: &mut Commands, path: &str, asset_server: &Res<AssetServer>) {
    spawn_entity_from_json(commands, &load_file_to_json(path), asset_server);
}

pub fn spawn_entity_from_json(commands: &mut Commands, input_json: &JsonValue, asset_server: &Res<AssetServer>) {
    // create entity
    let mut entity = commands.spawn_empty();

    // unpack json
    let components = &input_json["components"];

    // if components is an array, loop through each component
    if components.is_array() {
        for i in 0 .. components.len() {
            // add each component to the entity
            let bundle = convert_component_to_bundle(&components[i], asset_server);
            if bundle.is_ok() {
                bundle.unwrap().attach(&mut entity);
            } else {
                warn!("Failed to convert json to component with input {}", input_json)
            }
        }
    }
}

pub enum EntityBundle {
    Model(PbrBundle)
}

impl EntityBundle {
    fn attach(self, commands: &mut EntityCommands) {
        match self {
            Self::Model(bundle) => commands.insert(bundle)
        };
    }
}

fn convert_component_to_bundle(input_json: &JsonValue, asset_server: &Res<AssetServer>) -> Result<EntityBundle, String> {
    // unpack json
    let type_str = input_json["type"].as_str();

    // make sure unpacked info is correct
    if type_str.is_none() {
        return Err(format!("Could not grab type from component"));
    }
    let type_str = type_str.unwrap();

    //mesh: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Mesh0/Primitive0"),
    //material: asset_server.load("models/FlightHelmet/FlightHelmet.gltf#Material0"),

    // match the component and add it to the entity
    Ok(match type_str {
        "model" => {
            let target = input_json["model"].as_str().unwrap();
            EntityBundle::Model(
                PbrBundle {
                    mesh: asset_server.load(format!("{}#Mesh0/Primitive0", target).as_str()),
                    material: asset_server.load(format!("{}#Material0", target).as_str()),
                    ..Default::default()
                }
            )
        },
        _ => return Err(format!("Could not add type {}", type_str))
    })
}
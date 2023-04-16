use std::fmt::format;

use bevy::{prelude::{Bundle, NodeBundle}};
use json::{ JsonValue };

pub fn convert_uifile_to_uibundle(path: &str) -> Result<impl Bundle, String> {
    let file_contents = std::fs::read_to_string(path).unwrap();
    let input_json = json::parse(file_contents.as_str()).unwrap();
    return convert_json_to_uibundle(input_json);
}

pub fn convert_json_to_uibundle(input_json: JsonValue) -> Result<impl Bundle, String> {
    return match input_json["type"].as_str().unwrap() {
        // "node" => println!("Node");
        "node" => Ok(convert_json_to_node_bundle(input_json)),
        _ => Err(format!("No type registered of name {}", input_json["type"]))
    }
}

fn convert_json_to_node_bundle(input_json: JsonValue) -> NodeBundle {
    return NodeBundle { ..Default::default() }
}
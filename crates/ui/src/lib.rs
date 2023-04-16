use bevy::{prelude::{Bundle, NodeBundle, Style, Size, UiRect, Val}};
use json::{ JsonValue };
use ui_utils::optional_color;

pub mod ui_utils;
pub mod enum_utils;

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
    return NodeBundle {
        style: Style {
            size: Size::width(Val::Px(200.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        background_color: optional_color(input_json, "background_color").into(),
        ..Default::default() 
    }
}

// TODO: convert json to style
//  - TODO: convert json to Val (https://docs.rs/bevy/latest/bevy/ui/enum.Val.html)
//  - TODO: convert json to UiRect (https://docs.rs/bevy/latest/bevy/ui/struct.UiRect.html)
//  - todo: convert json to size (https://docs.rs/bevy/latest/bevy/ui/struct.Size.html)
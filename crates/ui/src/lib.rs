use std::panic::RefUnwindSafe;

use bevy::prelude::*;
use enum_utils::{focus_policy, visibility, zindex};
use json::{ JsonValue };
use ui_utils::*;

pub mod ui_utils;
pub mod enum_utils;

pub fn convert_uifile_to_uibundle<T: Bundle>(path: &str) -> Box<impl Bundle> {
    let file_contents = std::fs::read_to_string(path).unwrap();
    let input_json = json::parse(file_contents.as_str()).unwrap();
    return convert_json_to_uibundle(input_json);
}

pub fn convert_json_to_uibundle(input_json: JsonValue) -> Box<impl Bundle> {
    let type_str = input_json["type"].as_str().unwrap();

    return match type_str {
        "node" => Box::new(NodeBundle {
            style: optional_style(&input_json, "style"),
            background_color: optional_color(&input_json, "background_color").into(),
            focus_policy: focus_policy(optional_string(&input_json, "focus_policy")),
            transform: optional_transform(&input_json, "transform"),
            visibility: visibility(optional_string(&input_json, "visibility")),
            z_index: zindex(optional_string(&input_json, "z_index")),
            ..Default::default() 
        }),
        // "text" => Box::new(TextBundle {
        //     text: Text::from_section(optional_string(&input_json, "text"), TextStyle::default()),
        //     calculated_size: optional_calculated_size(&input_json, "calculated_size"),
        //     style: optional_style(&input_json, "style"),
        //     background_color: optional_color(&input_json, "background_color").into(),
        //     focus_policy: focus_policy(optional_string(&input_json, "focus_policy")),
        //     transform: optional_transform(&input_json, "transform"),
        //     visibility: visibility(optional_string(&input_json, "visibility")),
        //     z_index: zindex(optional_string(&input_json, "z_index")),
        //     ..Default::default() 
        //  }),
         _ => panic!("bob")
    }
}

fn convert_json_to_node_bundle(input_json: JsonValue) -> NodeBundle {
    return NodeBundle {
        style: optional_style(&input_json, "style"),
        background_color: optional_color(&input_json, "background_color").into(),
        focus_policy: focus_policy(optional_string(&input_json, "focus_policy")),
        transform: optional_transform(&input_json, "transform"),
        visibility: visibility(optional_string(&input_json, "visibility")),
        z_index: zindex(optional_string(&input_json, "z_index")),
        ..Default::default() 
    }
}

fn convert_json_to_text_bundle(input_json: JsonValue) -> TextBundle {
    return TextBundle {
        text: Text::from_section(optional_string(&input_json, "text"), TextStyle::default()),
        calculated_size: optional_calculated_size(&input_json, "calculated_size"),
        style: optional_style(&input_json, "style"),
        background_color: optional_color(&input_json, "background_color").into(),
        focus_policy: focus_policy(optional_string(&input_json, "focus_policy")),
        transform: optional_transform(&input_json, "transform"),
        visibility: visibility(optional_string(&input_json, "visibility")),
        z_index: zindex(optional_string(&input_json, "z_index")),
        ..Default::default() 
     }
}
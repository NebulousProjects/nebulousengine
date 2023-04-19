use bevy::{prelude::*, ecs::system::EntityCommands};
use enum_utils::{focus_policy, visibility, zindex};
use json::{ JsonValue };
use ui_utils::*;

pub mod ui_utils;
pub mod enum_utils;

pub fn add_ui_json_to_commands(input_json: &JsonValue, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    // create entity
    let mut entity = commands.spawn_empty();

    // add ui and children
    insert_json_ui_bundle(input_json, &mut entity, asset_server);
    insert_children(input_json, &mut entity, asset_server);

    // return final entity
    // return entity.id();
}

pub fn insert_children(input_json: &JsonValue, commands: &mut EntityCommands, asset_server: &Res<AssetServer>) {
    let children = &input_json["children"];
    commands.with_children(|builder| {
        for i in 0 .. children.len() {
            let json = &children[i];
            let mut entity = builder.spawn_empty();
            insert_json_ui_bundle(json, &mut entity, asset_server);
            insert_children(json, &mut entity, asset_server);
        }
    });
}

pub fn insert_json_ui_bundle(input_json: &JsonValue, commands: &mut EntityCommands, asset_server: &Res<AssetServer>) {
    println!("Drawing ui with type: {}", input_json["type"]);
    let bundle = gen_ui_bundle(input_json, asset_server);
    if bundle.is_ok() {
        bundle.unwrap().attach(commands);
    } else {
        println!("Drawing ui caused error: {}", bundle.err().unwrap());
    }
}

pub enum UiBundle {
    Node(NodeBundle),
    Text(TextBundle),
    Image(ImageBundle)
}

impl UiBundle  {
    fn attach(self, commands: &mut EntityCommands) {
        match self {
            Self::Node(bundle) => commands.insert(bundle.clone()),
            Self::Text(bundle) => commands.insert(bundle.clone()),
            Self::Image(bundle) => commands.insert(bundle.clone())
        };
    }
}

#[derive(Debug)]
pub enum JsonToUiErr {
    NoType,
    CannotConvertToStr,
    UnknownType,
}

fn gen_ui_bundle(input_json: &JsonValue, asset_server: &Res<AssetServer>) -> Result<UiBundle, String> {
    // if !value.has_key("type") { JsonToUiErr::NoType }

    let type_str = input_json["type"].as_str().unwrap();

    Ok(match type_str {
        "node" => UiBundle::Node(
            NodeBundle {
                style: optional_style(&input_json, "style"),
                background_color: optional_color(&input_json, "background_color").into(),
                focus_policy: focus_policy(optional_string(&input_json, "focus_policy")),
                transform: optional_transform(&input_json, "transform"),
                visibility: visibility(optional_string(&input_json, "visibility")),
                z_index: zindex(optional_string(&input_json, "z_index")),
                ..Default::default() 
            }
        ),
        "text" => UiBundle::Text(
            TextBundle {
                text: Text::from_section(optional_string(&input_json, "text"), TextStyle {
                    font: asset_server.load("./fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                }),
                calculated_size: optional_calculated_size(&input_json, "calculated_size"),
                style: optional_style(&input_json, "style"),
                background_color: optional_color(&input_json, "background_color").into(),
                focus_policy: focus_policy(optional_string(&input_json, "focus_policy")),
                transform: optional_transform(&input_json, "transform"),
                visibility: visibility(optional_string(&input_json, "visibility")),
                z_index: zindex(optional_string(&input_json, "z_index")),
                ..Default::default() 
            }
        ),
        "image" => UiBundle::Image(
            ImageBundle {
                image: optional_image(&input_json, asset_server, "image"),
                calculated_size: optional_calculated_size(&input_json, "calculated_size"),
                style: optional_style(&input_json, "style"),
                background_color: optional_color_default(&input_json, "background_color", Color::WHITE).into(),
                focus_policy: focus_policy(optional_string(&input_json, "focus_policy")),
                transform: optional_transform(&input_json, "transform"),
                visibility: visibility(optional_string(&input_json, "visibility")),
                z_index: zindex(optional_string(&input_json, "z_index")),
                ..Default::default() 
            }
        ),
        _ => return Err("Unknown type".to_string())
    })
}
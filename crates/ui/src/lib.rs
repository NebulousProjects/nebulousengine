use bevy::prelude::*;
use enum_utils::{focus_policy, visibility, zindex};
use json::{ JsonValue };
use ui_utils::*;

pub mod ui_utils;
pub mod enum_utils;

pub enum UiBundle {
    Node(NodeBundle),
    Text(TextBundle)
}

// TODO: Turn json into entities on spawn
// impl UiBundle  {
//     fn apply_to_entity(self, commands: &mut Commands) -> Entity {
//         let entity = commands.spawn_empty();
//         match self {
//             Self::Node(bundle) => entity.insert(bundle),
//             Self::Text(bundle) => entity.insert(bundle)
//         }
//         let result = entity.id();
//         result;
//     }
// }

pub enum JsonToUiErr {
    NoType,
    CannotConvertToStr,
    UnknownType,
}

impl TryFrom<JsonValue> for UiBundle {
    type Error = JsonToUiErr;

    fn try_from(input_json: JsonValue) -> Result<Self, Self::Error> {
        // if !value.has_key("type") { JsonToUiErr::NoType }

        let type_str = input_json["type"].as_str().unwrap();

        Ok(match type_str {
            "node" => Self::Node(
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
            "text" => Self::Text(
                TextBundle {
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
            ),
            _ => return Err(JsonToUiErr::UnknownType)
        })
    }
}
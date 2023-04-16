use bevy::{prelude::{Color}};
use json::{ JsonValue };
use std::u32;

pub fn optional_color(json_container: JsonValue, target: &str) -> Color {
    if json_container.has_key(target) { 
        let value = json_container[target].as_str().unwrap();
        return Color::rgb(
            u32::from_str_radix(&value[1..3], 16).unwrap() as f32 / 255.0, 
            u32::from_str_radix(&value[3..5], 16).unwrap() as f32 / 255.0, 
            u32::from_str_radix(&value[5..7], 16).unwrap() as f32 / 255.0
        ).into();
    } else {
        return Color::NONE.into();
    }
}

// pub fn optional_enum(json_container: JsonValue, target: &str) -> Result<JustifyContent, &str> {
//     if json_container.has_key(target) {
//         let value: JustifyContent = json_container[target].to_string().into();
//         return Ok(value);
//     } else {
//         return JustifyContent::End;
//     }
// }


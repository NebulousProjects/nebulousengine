use bevy::{prelude::*, reflect::Enum};
use serde::*;

pub fn serialize<S>(button: &MouseButton, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(button.variant_name())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<MouseButton, D::Error> where D: Deserializer<'de> {
    let value = String::deserialize(deserializer)?;
    let value = value.as_str();
    match value {
        "Left" => Ok(MouseButton::Left),
        "Right" => Ok(MouseButton::Right),
        "Middle" => Ok(MouseButton::Middle),
        _ => { error!("No mouse button registered with value {value}"); Ok(MouseButton::Left) }
    }
}
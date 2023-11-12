use bevy::{prelude::*, reflect::Enum};
use serde::*;

pub fn serialize<S>(button: &GamepadAxisType, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(button.variant_name())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<GamepadAxisType, D::Error> where D: Deserializer<'de> {
    let value = String::deserialize(deserializer)?;
    let value = value.as_str();
    match value {
        "LeftX" => Ok(GamepadAxisType::LeftStickX),
        "LeftY" => Ok(GamepadAxisType::LeftStickY),
        "LeftZ" => Ok(GamepadAxisType::LeftZ),
        "RightX" => Ok(GamepadAxisType::RightStickX),
        "RightY" => Ok(GamepadAxisType::RightStickY),
        "RightZ" => Ok(GamepadAxisType::RightZ),
        _ => { error!("No mouse button registered with value {value}"); Ok(GamepadAxisType::LeftStickX) }
    }
}
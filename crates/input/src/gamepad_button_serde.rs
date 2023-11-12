use bevy::{prelude::*, reflect::Enum};
use serde::*;

pub fn serialize<S>(button: &GamepadButtonType, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(button.variant_name())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<GamepadButtonType, D::Error> where D: Deserializer<'de> {
    let value = String::deserialize(deserializer)?;
    let value = value.as_str();
    match value {
        "South" => Ok(GamepadButtonType::South),
        "East" => Ok(GamepadButtonType::East),
        "North" => Ok(GamepadButtonType::North),
        "West" => Ok(GamepadButtonType::West),
        "C" => Ok(GamepadButtonType::C),
        "Z" => Ok(GamepadButtonType::Z),
        "LeftTrigger" => Ok(GamepadButtonType::LeftTrigger),
        "LeftTrigger2" => Ok(GamepadButtonType::LeftTrigger2),
        "RightTrigger" => Ok(GamepadButtonType::RightTrigger),
        "RightTrigger2" => Ok(GamepadButtonType::RightTrigger2),
        "Select" => Ok(GamepadButtonType::Select),
        "Start" => Ok(GamepadButtonType::Start),
        "Mode" => Ok(GamepadButtonType::Mode),
        "LeftThumb" => Ok(GamepadButtonType::LeftThumb),
        "RightThumb" => Ok(GamepadButtonType::RightThumb),
        "DPadUp" => Ok(GamepadButtonType::DPadUp),
        "DPadDown" => Ok(GamepadButtonType::DPadDown),
        "DPadLeft" => Ok(GamepadButtonType::DPadLeft),
        "DPadRight" => Ok(GamepadButtonType::DPadRight),
        _ => { error!("No mouse button registered with value {value}"); Ok(GamepadButtonType::East) }
    }
}
use bevy::prelude::*;
use json::JsonValue;

pub fn mouse_button(target: &JsonValue) -> Result<MouseButton, String> {
    if target.is_number() {
        Ok(MouseButton::Other(target.as_u16().unwrap()))
    } else if target.is_string() {
        let type_str = target.as_str();
        if type_str.is_none() {
            Err(format!("Could not get type string in mouse button!"))
        } else {
            match type_str.unwrap() {
                "left" => Ok(MouseButton::Left),
                "right" => Ok(MouseButton::Right),
                "middle" => Ok(MouseButton::Middle),
                _ => Err(format!("Unknown type {}", type_str.unwrap()))
            }
        }
    } else {
        Err("Could not decode mouse button".to_string())
    }
}

pub fn gamepad_button_type(target: &JsonValue) -> Result<GamepadButtonType, String> {
    if target.is_number() {
        Ok(GamepadButtonType::Other(target.as_u8().unwrap()))
    } else if target.is_string() {
        let type_str = target.as_str();
        if type_str.is_none() {
            Err(format!("Could not get type string in gamepad button type!"))
        } else {
            match type_str.unwrap() {
                "south" => Ok(GamepadButtonType::South),
                "east" => Ok(GamepadButtonType::East),
                "north" => Ok(GamepadButtonType::North),
                "west" => Ok(GamepadButtonType::West),
                "c" => Ok(GamepadButtonType::C),
                "z" => Ok(GamepadButtonType::Z),
                "left_trigger" => Ok(GamepadButtonType::LeftTrigger),
                "left_trigger_2" => Ok(GamepadButtonType::LeftTrigger2),
                "right_trigger" => Ok(GamepadButtonType::RightTrigger),
                "right_trigger_2" => Ok(GamepadButtonType::RightTrigger2),
                "select" => Ok(GamepadButtonType::Select),
                "start" => Ok(GamepadButtonType::Start),
                "mode" => Ok(GamepadButtonType::Mode),
                "left_thumb" => Ok(GamepadButtonType::LeftThumb),
                "right_thumb" => Ok(GamepadButtonType::RightThumb),
                "dpad_up" => Ok(GamepadButtonType::DPadUp),
                "dpad_down" => Ok(GamepadButtonType::DPadDown),
                "dpad_left" => Ok(GamepadButtonType::DPadLeft),
                "dpad_right" => Ok(GamepadButtonType::DPadRight),
                _ => Err(format!("Unknown type {}", type_str.unwrap()))
            }
        }
    } else {
        Err("Could not decode gamepad button type".to_string())
    }
}

pub fn gamepad_axis_type(target: &JsonValue) -> Result<GamepadAxisType, String> {
    if target.is_number() {
        Ok(GamepadAxisType::Other(target.as_u8().unwrap()))
    } else if target.is_string() {
        let type_str = target.as_str();
        if type_str.is_none() {
            Err(format!("Could not get type string in gamepad axis type!"))
        } else {
            match type_str.unwrap() {
                "left_stick_x" => Ok(GamepadAxisType::LeftStickX),
                "left_stick_y" => Ok(GamepadAxisType::LeftStickY),
                "left_z" => Ok(GamepadAxisType::LeftZ),
                "right_stick_x" => Ok(GamepadAxisType::RightStickX),
                "right_stick_y" => Ok(GamepadAxisType::RightStickY),
                "right_z" => Ok(GamepadAxisType::RightZ),
                _ => Err(format!("Unknown type {}", type_str.unwrap()))
            }
        }
    } else {
        Err("Could not decode gamepad axis".to_string())
    }
}
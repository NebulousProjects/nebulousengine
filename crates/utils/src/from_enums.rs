use bevy::prelude::*;

pub fn from_mouse_button(button: &MouseButton) -> String {
    match button {
        MouseButton::Left => "Left".to_string(),
        MouseButton::Right => "Right".to_string(),
        MouseButton::Middle => "Middle".to_string(),
        MouseButton::Other(other) => format!("{}", other)
    }
}

pub fn from_gamepad_button_type(button: &GamepadButtonType) -> String {
    match button {
        GamepadButtonType::South => "South".to_string(),
        GamepadButtonType::North => "North".to_string(),
        GamepadButtonType::East => "East".to_string(),
        GamepadButtonType::West => "West".to_string(),
        GamepadButtonType::C => "C".to_string(),
        GamepadButtonType::Z => "Z".to_string(),
        GamepadButtonType::LeftTrigger => "L1".to_string(),
        GamepadButtonType::LeftTrigger2 => "L2".to_string(),
        GamepadButtonType::RightTrigger => "R1".to_string(),
        GamepadButtonType::RightTrigger2 => "R2".to_string(),
        GamepadButtonType::Select => "Select".to_string(),
        GamepadButtonType::Start => "Start".to_string(),
        GamepadButtonType::Mode => "Mode".to_string(),
        GamepadButtonType::LeftThumb => "L3".to_string(),
        GamepadButtonType::RightThumb => "R3".to_string(),
        GamepadButtonType::DPadUp => "Up".to_string(),
        GamepadButtonType::DPadDown => "Down".to_string(),
        GamepadButtonType::DPadLeft => "Left".to_string(),
        GamepadButtonType::DPadRight => "Right".to_string(),
        GamepadButtonType::Other(other) => format!("{}", other),
    }
}

pub fn from_gamepad_axis_type(axis: &GamepadAxisType) -> String {
    match axis {
        GamepadAxisType::LeftStickX => "Left Stick Horizontal".to_string(),
        GamepadAxisType::LeftStickY => "Left Stick Vertical".to_string(),
        GamepadAxisType::LeftZ => "Left Z".to_string(),
        GamepadAxisType::RightStickX => "Right Stick Horizontal".to_string(),
        GamepadAxisType::RightStickY => "Right Stick Vertical".to_string(),
        GamepadAxisType::RightZ => "Right Z".to_string(),
        GamepadAxisType::Other(other) => format!("{}", other),
    }
}

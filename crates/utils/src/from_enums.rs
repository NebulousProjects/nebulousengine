use bevy::prelude::*;

pub fn from_mouse_button(button: &MouseButton) -> String {
    match button {
        MouseButton::Left => "left".to_string(),
        MouseButton::Right => "right".to_string(),
        MouseButton::Middle => "center".to_string(),
        MouseButton::Other(other) => format!("{}", other)
    }
}

pub fn from_mouse_button_english(button: &MouseButton) -> String {
    match button {
        MouseButton::Left => "Left".to_string(),
        MouseButton::Right => "Right".to_string(),
        MouseButton::Middle => "Middle".to_string(),
        MouseButton::Other(other) => format!("{}", other)
    }
}

pub fn from_gamepad_button_type(button: &GamepadButtonType) -> String {
    match button {
        GamepadButtonType::South => "south".to_string(),
        GamepadButtonType::North => "north".to_string(),
        GamepadButtonType::East => "east".to_string(),
        GamepadButtonType::West => "west".to_string(),
        GamepadButtonType::C => "c".to_string(),
        GamepadButtonType::Z => "z".to_string(),
        GamepadButtonType::LeftTrigger => "left_trigger".to_string(),
        GamepadButtonType::LeftTrigger2 => "left_trigger_2".to_string(),
        GamepadButtonType::RightTrigger => "right_trigger".to_string(),
        GamepadButtonType::RightTrigger2 => "right_trigger_2".to_string(),
        GamepadButtonType::Select => "select".to_string(),
        GamepadButtonType::Start => "start".to_string(),
        GamepadButtonType::Mode => "mode".to_string(),
        GamepadButtonType::LeftThumb => "left_thumb".to_string(),
        GamepadButtonType::RightThumb => "right_thumb".to_string(),
        GamepadButtonType::DPadUp => "dpad_up".to_string(),
        GamepadButtonType::DPadDown => "dpad_down".to_string(),
        GamepadButtonType::DPadLeft => "dpad_left".to_string(),
        GamepadButtonType::DPadRight => "dpad_right".to_string(),
        GamepadButtonType::Other(other) => format!("{}", other),
    }
}

pub fn from_gamepad_button_type_english(button: &GamepadButtonType) -> String {
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
        GamepadButtonType::DPadUp => "DPad Up".to_string(),
        GamepadButtonType::DPadDown => "DPad Down".to_string(),
        GamepadButtonType::DPadLeft => "DPad Left".to_string(),
        GamepadButtonType::DPadRight => "DPad Right".to_string(),
        GamepadButtonType::Other(other) => format!("{}", other),
    }
}

pub fn from_gamepad_axis_type(axis: &GamepadAxisType) -> String {
    match axis {
        GamepadAxisType::LeftStickX => "left_stick_x".to_string(),
        GamepadAxisType::LeftStickY => "left_stick_y".to_string(),
        GamepadAxisType::LeftZ => "left_z".to_string(),
        GamepadAxisType::RightStickX => "right_stick_x".to_string(),
        GamepadAxisType::RightStickY => "right_stick_y".to_string(),
        GamepadAxisType::RightZ => "right_z".to_string(),
        GamepadAxisType::Other(other) => format!("{}", other),
    }
}

pub fn from_gamepad_axis_type_english(axis: &GamepadAxisType) -> String {
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

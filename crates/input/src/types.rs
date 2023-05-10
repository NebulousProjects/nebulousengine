use bevy::{prelude::*, input::mouse::{MouseMotion}};
use json::JsonValue;
use nebulousengine_utils::{insert_json, from_enums::{from_mouse_button, from_gamepad_button_type, from_gamepad_axis_type}};

use crate::GamepadContainer;

pub struct InputPressedEvent {
    pub container: String,
    pub name: String,
    pub value: f32
}
pub struct InputReleasedEvent {
    pub container: String,
    pub name: String,
    pub value: f32
}
pub struct InputChangedEvent {
    pub container: String,
    pub name: String,
    pub value: f32
}

pub struct InputValue {
    pub press_threshold: f32,
    pub descriptions: Vec<InputDescription>,
    pub value: f32
}

pub enum InputDescription {
    Single { input_type: InputType },
    Double { positive_type: InputType, negative_type: InputType }
}

pub enum InputType {
    Keyboard(ScanCode),
    MouseMotionX(),
    MouseMotionY(),
    MouseButton(MouseButton),
    GamepadButton(GamepadButtonType),
    GamepadAxis(GamepadAxisType)
}

impl InputType {
    pub fn to_json(&self) -> JsonValue {
        match self {
            InputType::MouseMotionX() => {
                let mut object =  JsonValue::new_object();
                insert_json(&mut object, "type", "mouse_motion_x".into());
                object
            },
            InputType::MouseMotionY() => {
                let mut object =  JsonValue::new_object();
                insert_json(&mut object, "type", "mouse_motion_y".into());
                object
            },
            InputType::Keyboard(keycode) => {
                let mut object = JsonValue::new_object();
                insert_json(&mut object, "type", "keyboard".into());
                insert_json(&mut object, "keycode", keycode.0.into());
                object
            },
            InputType::MouseButton(button) => {
                let mut object = JsonValue::new_object();
                insert_json(&mut object, "type", "mouse_button".into());
                insert_json(&mut object, "button", from_mouse_button(button).into());
                object
            },
            InputType::GamepadButton(button) => {
                let mut object = JsonValue::new_object();
                insert_json(&mut object, "type", "gamepad_button".into());
                insert_json(&mut object, "button", from_gamepad_button_type(button).into());
                object
            },
            InputType::GamepadAxis(axis) => {
                let mut object = JsonValue::new_object();
                insert_json(&mut object, "type", "gamepad_axis".into());
                insert_json(&mut object, "axis", from_gamepad_axis_type(axis).into());
                object
            }
        }
    }
}

fn eval_input_type(
    input_type: &InputType,
    primary_window: &Window,
    keys: &Res<Input<ScanCode>>,
    mouse_buttons: &Res<Input<MouseButton>>,
    pad_buttons: &Res<Input<GamepadButton>>,
    pad_axes: &Res<Axis<GamepadAxis>>,
    mouse_motion: &mut EventReader<MouseMotion>,
    gamepad_container: &Option<Res<GamepadContainer>>
) -> f32 {
    return match input_type {
        InputType::Keyboard(keycode) => {
            if keys.pressed(*keycode) { 1.0 } else { 0.0 }
        },
        InputType::MouseMotionX() => {
            return if mouse_motion.is_empty() { 0.0 } else {
                let mut output = 0.0;
                
                for event in mouse_motion.iter() {
                    output += event.delta.x;
                }

                output / primary_window.height() // yes this is supposed to be height not width
            }
        },
        InputType::MouseMotionY() => {
            return if mouse_motion.is_empty() { 0.0 } else {
                let mut output = 0.0;
                
                for event in mouse_motion.iter() {
                    output += event.delta.y;
                }

                output / primary_window.height()
            }
        },
        InputType::MouseButton(button) => {
            if mouse_buttons.pressed(*button) { 1.0 } else { 0.0 }
        },
        InputType::GamepadButton(button) => {
            if gamepad_container.is_some() {
                let container = gamepad_container.as_deref().unwrap();
                if pad_buttons.pressed(GamepadButton { gamepad: container.0, button_type: *button }) { 1.0 } else { 0.0 }
            } else { 0.0 }
        },
        InputType::GamepadAxis(axis) => {
            if gamepad_container.is_some() {
                let container = gamepad_container.as_deref().unwrap();
                let option = pad_axes.get(GamepadAxis { gamepad: container.0, axis_type: *axis });
                if option.is_some() { option.unwrap() } else { 0.0 }
            } else { 0.0 }
        }
    }
}

pub fn eval_description(
    description: &InputDescription,
    primary_window: &Window,
    keys: &Res<Input<ScanCode>>,
    mouse_buttons: &Res<Input<MouseButton>>,
    pad_buttons: &Res<Input<GamepadButton>>,
    pad_axes: &Res<Axis<GamepadAxis>>,
    mouse_motion: &mut EventReader<MouseMotion>,
    gamepad_container: &Option<Res<GamepadContainer>>,
) -> f32 {
    match description {
        InputDescription::Single { input_type } => {
            eval_input_type(input_type, primary_window, keys, mouse_buttons, pad_buttons, pad_axes, mouse_motion, gamepad_container)
        },
        InputDescription::Double { positive_type, negative_type } => {
            eval_input_type(positive_type, primary_window, keys, mouse_buttons, pad_buttons, pad_axes, mouse_motion, gamepad_container) 
            - eval_input_type(negative_type, primary_window, keys, mouse_buttons, pad_buttons, pad_axes, mouse_motion, gamepad_container)
        }
    }
}

pub fn eval_value(
    rule: &InputValue,
    primary_window: &Window,
    keys: &Res<Input<ScanCode>>,
    mouse_buttons: &Res<Input<MouseButton>>,
    pad_buttons: &Res<Input<GamepadButton>>,
    pad_axes: &Res<Axis<GamepadAxis>>,
    mouse_motion: &mut EventReader<MouseMotion>,
    gamepad_container: &Option<Res<GamepadContainer>>,
) -> f32 {
    let mut output = 0.0;
    let mut count = 0.0;

    for description in rule.descriptions.iter() {
        let eval = eval_description(description, primary_window, keys, mouse_buttons, pad_buttons, pad_axes, mouse_motion, gamepad_container);
        if eval != 0.0 {
            output += eval;
            count += 1.0;
        }
    }

    if count == 0.0 { 0.0 } else { output / count }
}
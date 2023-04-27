use bevy::{prelude::*, input::mouse::MouseMotion};

pub struct InputPressedEvent {
    pub name: String,
    pub value: f32
}
pub struct InputReleasedEvent {
    pub name: String,
    pub value: f32
}

pub struct InputValue {
    pub rule: InputRule,
    pub value: f32
}

pub struct InputRule {
    pub press_threshold: f32,
    pub descriptions: Vec<InputDescription>
}

pub enum InputDescription {
    Scalar { input_type: InputType },
    Axis { positive_type: InputType, negative_type: InputType }
}

pub enum InputType {
    Keyboard(KeyCode),
    MouseMotionX(),
    MouseMotionY(),
}

fn eval_input_type(
    input_type: &InputType,
    keys: &Res<Input<KeyCode>>,
    mouse_motion: &mut EventReader<MouseMotion>,
    primary_window: &Window
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
        }
    }
}

pub fn eval_description(
    description: &InputDescription,
    keys: &Res<Input<KeyCode>>,
    mouse_motion: &mut EventReader<MouseMotion>,
    primary_window: &Window
) -> f32 {
    match description {
        InputDescription::Scalar { input_type } => {
            eval_input_type(input_type, keys, mouse_motion, primary_window)
        },
        InputDescription::Axis { positive_type, negative_type } => {
            eval_input_type(positive_type, keys, mouse_motion, primary_window) - eval_input_type(negative_type, keys, mouse_motion, primary_window)
        }
    }
}

pub fn eval_rule(
    rule: &InputRule,
    keys: &Res<Input<KeyCode>>,
    mouse_motion: &mut EventReader<MouseMotion>,
    primary_window: &Window
) -> f32 {
    let mut output = 0.0;
    let mut count = 0.0;

    for description in rule.descriptions.iter() {
        let eval = eval_description(description, keys, mouse_motion, primary_window);
        if eval != 0.0 {
            output += eval;
            count += 1.0;
        }
    }

    output / count
}
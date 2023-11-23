use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, utils::HashMap};
use serde::*;

// A serializable version of input map
#[derive(Serialize, Deserialize, Component, TypeUuid, TypePath, Asset, Debug, Default)]
#[uuid = "135601b6-2de3-4497-8f4b-3f4841948584"]
pub struct InputDescription {
    pub elements: HashMap<String, Vec<InputType>>
}

impl InputDescription {
    pub fn insert(&mut self, name: impl Into<String>, inputs: Vec<InputType>) -> &mut Self {
        self.elements.insert(name.into(), inputs);
        self
    }

    pub fn remove(&mut self, name: impl Into<String>) -> &mut Self {
        self.elements.remove(&name.into());
        self
    }

    pub fn get(&mut self, name: impl Into<String>) -> Option<&Vec<InputType>> {
        self.elements.get(&name.into())
    }

    pub fn get_mut(&mut self, name: impl Into<String>) -> Option<&mut Vec<InputType>> {
        self.elements.get_mut(&name.into())
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct InputValues {
    pub values: HashMap<String, f32>
}
impl InputValues {
    pub fn get(&self, name: &String) -> f32 {
        return *self.values.get(name).unwrap_or(&0.0);
    }

    pub fn set(&mut self, name: String, value: f32) {
        self.values.insert(name, value);
    }
}

// Represents all possible input, types, scalar (0 -> 1, 1 input) or axis (-1 -> 1, 2 inputs)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InputType {
    SCALAR { element: InputElement },
    AXIS { positive: InputElement, negative: InputElement }
}

// Represents an input element like a keyboard key or a mouse axis
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InputElement {
    Keyboard {
        #[serde(with = "crate::keycode_serde")]
        key: KeyCode
    },
    Mouse {
        #[serde(with = "crate::mouse_button_serde")]
        button: MouseButton
    },
    GamepadButton {
        #[serde(with = "crate::gamepad_button_serde")]
        button: GamepadButtonType
    },
    GamepadAxis {
        #[serde(with = "crate::gamepad_axis_serde")]
        axis: GamepadAxisType,
        #[serde(default = "mult_default")]
        mult: f32
    }
}

fn mult_default() -> f32 { 1. }
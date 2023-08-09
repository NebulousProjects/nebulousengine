use bevy::prelude::*;

use crate::serializables::*;

#[derive(Component)]
pub struct Ui {
    pub spawnable: UiSpawnable,
    pub commands: Vec<UiCommand>
}

pub enum UiSpawnable {
    Handle { handle: Handle<UiElement> },
    Direct { element: UiElement }
}

pub struct UiCommand {
    pub target: String,
    pub command: UiCommandType
}

pub enum UiCommandType {
    Add { spawnable: UiSpawnable },
    Remove,
    ModText { new_text: Text },
    ModBGColor { color: Color },
    ModBorderColor { color: Color }
}

impl Ui {
    // functions to make creating commands easier
    pub fn from_handle(handle: Handle<UiElement>) -> Self {
        Self { spawnable: UiSpawnable::Handle { handle }, commands: Vec::new() }
    }

    pub fn from_element(element: UiElement) -> Self {
        Self { spawnable: UiSpawnable::Direct { element }, commands: Vec::new() }
    }
}
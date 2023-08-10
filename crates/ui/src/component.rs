use bevy::prelude::*;

use crate::serializables::*;

#[derive(Bundle, Default)]
pub struct UiBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub ui: Ui
}

#[derive(Component, Default)]
pub struct Ui {
    pub spawnable: UiSpawnable,
    pub commands: Vec<UiCommand>
}

#[derive(Default)]
pub enum UiSpawnable {
    Handle { handle: Handle<UiElement> },
    Direct { element: UiElement },
    #[default]
    Empty
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

    // functions to make commands easy
    pub fn add_element(&mut self, target: String, new: UiElement) {
        self.commands.push(UiCommand { target, command: UiCommandType::Add { spawnable: UiSpawnable::Direct { element: new } } });
    }

    pub fn remove_element(&mut self, target: String) {
        self.commands.push(UiCommand { target, command: UiCommandType::Remove });
    }

    pub fn set_text(&mut self, target: String, text: Text) {
        self.commands.push(UiCommand { target, command: UiCommandType::ModText { new_text: text } });
    }
}
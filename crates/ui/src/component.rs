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

#[derive(Default, Clone)]
pub enum UiSpawnable {
    Handle { handle: Handle<UiElement> },
    Direct { element: UiElement },
    #[default]
    Empty
}

#[derive(Clone)]
pub struct UiCommand {
    pub target: String,
    pub command: UiCommandType
}

#[derive(Clone)]
pub enum UiCommandType {
    Add { spawnable: UiSpawnable },
    Remove,
    ModText { new_text: Text },
    ModBGColor { color: Color },
    ModBorderColor { color: Color }
}

#[derive(Component)]
pub struct ScrollList {
    pub amount: f32
}

impl Default for ScrollList {
    fn default() -> Self {
        Self { amount: 0. }
    }
}

#[derive(Component)]
pub struct Collapsible {
    pub collapsed: bool
}

impl Default for Collapsible {
    fn default() -> Self {
        Self { collapsed: false }
    }
}

// marker component that markes a entity as it should not be hidden if its direct parent is a collapsable
#[derive(Component)]
pub struct NoCollapse;

impl Ui {
    // functions to make creating commands easier
    pub fn from_handle(handle: Handle<UiElement>) -> Self {
        Self { spawnable: UiSpawnable::Handle { handle }, commands: Vec::new() }
    }

    pub fn from_element(element: UiElement) -> Self {
        Self { spawnable: UiSpawnable::Direct { element }, commands: Vec::new() }
    }

    pub fn from_spawnable(spawnable: UiSpawnable) -> Self {
        Self { spawnable, commands: Vec::new() }
    }

    // functions to make commands easy
    pub fn add_element(&mut self, target: impl Into<String>, new: UiElement) {
        self.commands.push(UiCommand { target: target.into(), command: UiCommandType::Add { spawnable: UiSpawnable::Direct { element: new } } });
    }

    pub fn remove_element(&mut self, target: impl Into<String>) {
        self.commands.push(UiCommand { target: target.into(), command: UiCommandType::Remove });
    }

    pub fn set_text(&mut self, target: impl Into<String>, text: Text) {
        self.commands.push(UiCommand { target: target.into(), command: UiCommandType::ModText { new_text: text } });
    }
}
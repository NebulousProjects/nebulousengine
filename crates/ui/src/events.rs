use bevy::prelude::*;

use crate::UIID;

// UI events resource
#[derive(Resource, Default, Debug, Clone)]
pub struct UIEvents {
    just_pressed: Vec<String>,
    just_released: Vec<String>,
    pressed: Vec<String>
}

impl UIEvents {
    pub fn is_pressed(&self, name: impl Into<String>) -> bool { self.pressed.contains(&name.into()) }
    pub fn just_pressed(&self, name: impl Into<String>) -> bool { self.just_pressed.contains(&name.into()) }
    pub fn just_released(&self, name: impl Into<String>) -> bool { self.just_released.contains(&name.into()) }

    pub(crate) fn reset_button_events(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub(crate) fn trigger_button_pressed(&mut self, name: &String) {
        self.just_pressed.push(name.clone());
        self.pressed.push(name.clone());
    }

    pub(crate) fn trigger_button_released(&mut self, name: &String) {
        self.just_released.push(name.clone());
        self.pressed.retain(|a| a != name);
    }
}

// plugin to setup resource and systems for UI events
pub struct UIEventsPlugin;
impl Plugin for UIEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UIEvents>()
            .add_systems(PreUpdate, handle_interaction_inputs);
    }
}

fn handle_interaction_inputs(
    mut events: ResMut<UIEvents>,
    query: Query<(&UIID, &Interaction), Changed<Interaction>>
) {
    // reset events
    events.reset_button_events();

    // press and release buttons via their IDs
    query.for_each(|(id, interaction)| {
        match interaction {
            Interaction::Pressed => events.trigger_button_pressed(&id.0),
            Interaction::None | Interaction::Hovered => if events.is_pressed(&id.0) { events.trigger_button_released(&id.0) },
        }
    });
}

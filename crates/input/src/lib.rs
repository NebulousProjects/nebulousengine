use bevy::{prelude::*, input::mouse::MouseMotion};
use std::collections::*;

use types::*;

pub mod types;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update)
            .insert_resource(Inputs { values: HashMap::new() })
            .add_event::<InputPressedEvent>()
            .add_event::<InputReleasedEvent>();
    }
}

fn update(
    // input styff
    mut inputs: ResMut<Inputs>,
    mut pressed_events: EventWriter<InputPressedEvent>,
    mut released_events: EventWriter<InputReleasedEvent>,

    // bevy input stuff
    windows: Query<&Window>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>
) {
    let primary_window = windows.single();

    // loop through all active input rules
    for (name, value) in inputs.values.iter_mut() {
        // get new and old values
        let old_value = value.value;
        let new_value = eval_rule(&value.rule, &keys, &mut mouse_motion, primary_window);

        // if the value has been pressed, broadcast event
        if old_value.abs() < value.rule.press_threshold && new_value.abs() >= value.rule.press_threshold {
            pressed_events.send(InputPressedEvent {
                name: name.clone(),
                value: new_value
            });
        }

        // if the value has been released, broadcast event
        if old_value.abs() >= value.rule.press_threshold && new_value.abs() < value.rule.press_threshold {
            released_events.send(InputReleasedEvent {
                name: name.clone(),
                value: new_value
            });
        }

        // update the saved values
        value.value = new_value;
    }
}

#[derive(Resource)]
pub struct Inputs {
    values: HashMap<String, InputValue>
}
impl Inputs {
    pub fn insert_or_update_input(&mut self, name: String, value: InputValue) {
        self.values.insert(name, value);
    }

    pub fn remove_input(&mut self, name: &String) {
        self.values.remove(name);
    }

    pub fn get_value(&mut self, name: &String) -> Result<f32, String> {
        let value = self.values.get(name);
        if value.is_some() { Ok(value.unwrap().value) } 
        else { Err(format!("Could not find input with name {}", name)) }
    }

    pub fn get_value_or_default(&mut self, name: &String, default: f32) -> f32 {
        let output = self.get_value(name);
        if output.is_ok() { output.unwrap() } else { default }
    }
}
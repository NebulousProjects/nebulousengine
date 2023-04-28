use bevy::{prelude::*, input::{mouse::MouseMotion, gamepad::{GamepadEvent, GamepadConnection}}};
use std::collections::*;

use types::*;

pub mod types;

#[derive(Resource)]
pub struct GamepadContainer(Gamepad);

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(gamepad_update)
            .add_system(update)
            .insert_resource(Inputs { values: HashMap::new() })
            .add_event::<InputPressedEvent>()
            .add_event::<InputReleasedEvent>()
            .add_event::<InputChangedEvent>();
    }
}

fn gamepad_update(
    mut commands: Commands,
    mut gamepad_evr: EventReader<GamepadEvent>,
    gamepad_container: Option<Res<GamepadContainer>>
) {
    // loop through all game pad events and get connection events
    let mut already_pushed = false;
    for e in gamepad_evr.iter() {
        match e {
            GamepadEvent::Axis(_) => {},
            GamepadEvent::Button(_) => {},
            GamepadEvent::Connection(event) => {
                // get gamepad id
                let id = event.gamepad;

                // handle connect and disconnect
                match &event.connection {
                    // save new connection if necessary
                    GamepadConnection::Connected(info) => {
                        println!("New gamepad connected with ID: {:?}, name: {}", id, info.name);

                        // if we don't have any gamepad yet, use this one
                        if gamepad_container.is_none() && !already_pushed {
                            commands.insert_resource(GamepadContainer(id));
                            already_pushed = true;
                        }
                    },

                    // disconnect game pad
                    GamepadConnection::Disconnected => {
                        println!("Lost gamepad connection with ID: {:?}", id);
        
                        // if it's the one we previously associated with the player,
                        // disassociate it:
                        if let Some(GamepadContainer(old_id)) = gamepad_container.as_deref() {
                            if *old_id == id {
                                commands.remove_resource::<GamepadContainer>();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update(
    // input styff
    mut inputs: ResMut<Inputs>,
    mut pressed_events: EventWriter<InputPressedEvent>,
    mut released_events: EventWriter<InputReleasedEvent>,
    mut changed_events: EventWriter<InputChangedEvent>,

    // bevy input stuff
    windows: Query<&Window>,
    keys: Res<Input<KeyCode>>,
    pad_buttons: Res<Input<GamepadButton>>,
    pad_axes: Res<Axis<GamepadAxis>>,
    mut mouse_motion: EventReader<MouseMotion>,
    gamepad_container: Option<Res<GamepadContainer>>
) {
    let primary_window = windows.single();

    // loop through all active input rules
    for (name, value) in inputs.values.iter_mut() {
        // get new and old values
        let old_value = value.value;
        let new_value = eval_rule(&value.rule, primary_window, &keys, &pad_buttons, &pad_axes, &mut mouse_motion, &gamepad_container);

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

        if new_value != old_value {
            changed_events.send(InputChangedEvent { name: name.clone(), value: new_value });
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
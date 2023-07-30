use bevy::{prelude::*, utils::HashMap, asset::{AssetLoader, LoadedAsset}};
use structs::*;

pub mod structs;
mod keycode_serde;
mod mouse_button_serde;
mod gamepad_axis_serde;
mod gamepad_button_serde;

pub struct ConfigurableInputPlugin;
impl Plugin for ConfigurableInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<InputPressedEvent>()
            .add_event::<InputDepressedEvent>()
            .add_event::<InputChangedEvent>()
            .add_asset::<InputDescription>()
            .init_asset_loader::<InputLoader>()
            .add_systems(Update, (spawn_input_value, update_inputs));
    }
}

// Asset loader for loading input files
#[derive(Default)]
pub struct InputLoader;
impl AssetLoader for InputLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // load content
            let content = std::str::from_utf8(bytes);
            if content.is_err() { error!("Failed to load input json!"); return Err(bevy::asset::Error::msg("Failed to load json")) }
            let content = content.unwrap();
            
            // load description
            let description: Result<InputDescription, serde_json::Error> = serde_json::from_str(content);
            if description.is_err() { error!("Failed to load input description from json"); return Err(bevy::asset::Error::msg("Failed to load description")) }
            
            // load final input map
            load_context.set_default_asset(LoadedAsset::new(description.unwrap()));
            
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["input"]
    }
}

// System that spawns input values component to all input descriptions
fn spawn_input_value(
    mut commands: Commands,
    mut inputs: Query<(Entity, With<Handle<InputDescription>>, Without<InputValues>)>
) {
    inputs.for_each_mut(|(entity, _, _)| {
        commands.entity(entity).insert(InputValues { values: HashMap::new() });
    })
}

// System that loads all active input maps
fn update_inputs(
    // general
    mut inputs: Query<(&Handle<InputDescription>, &mut InputValues)>,
    descriptions: Res<Assets<InputDescription>>,

    // inputs
    keycodes: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    gamepad_buttons: Res<Input<GamepadButton>>,
    gamepad_axis: Res<Axis<GamepadAxis>>,

    // events
    mut pressed_events: EventWriter<InputPressedEvent>,
    mut depressed_events: EventWriter<InputDepressedEvent>,
    mut changed_events: EventWriter<InputChangedEvent>
) {
    inputs.for_each_mut(|(description_handle, mut values)| {
        // make sure description handle exists
        if !descriptions.contains(description_handle) { return }

        // load description
        let description = descriptions.get(description_handle);
        if description.is_none() { return }
        let description = description.unwrap();

        // for each description element and read and save its input
        description.elements.iter().for_each(|(name, input_types)| {
            // get sum with a max of 1 of all inputs
            let value: f32 = input_types.iter().map(|input_type| {
                match input_type {
                    InputType::SCALAR { element } => 
                        input_element_to_f32(element, &keycodes, &mouse_buttons, &gamepad_buttons, &gamepad_axis),
                    InputType::AXIS { positive, negative } => 
                        input_element_to_f32(positive, &keycodes, &mouse_buttons, &gamepad_buttons, &gamepad_axis) - 
                            input_element_to_f32(negative, &keycodes, &mouse_buttons, &gamepad_buttons, &gamepad_axis),
                }
            }).sum();
            let value = value.min(1.);

            // get last value
            let old_value = values.get(&name);
            
            // pressed events
            if old_value < 1. && value >= 1. {
                pressed_events.send(InputPressedEvent { name: name.clone(), value });
            }

            // depressed events
            if old_value >= 1. && value < 1. {
                depressed_events.send(InputDepressedEvent { name: name.clone(), value });
            }

            // change events
            if old_value != value {
                changed_events.send(InputChangedEvent { name: name.clone(), value });
            }

            // update values map with the resulting value
            values.set(name.clone(), value);
        });
    })
}

// A function that converts a input element to a f32
fn input_element_to_f32(
    element: &InputElement,
    keycodes: &Res<Input<KeyCode>>,
    mouse_buttons: &Res<Input<MouseButton>>,
    gamepad_buttons: &Res<Input<GamepadButton>>,
    gamepad_axis: &Res<Axis<GamepadAxis>>
) -> f32 {
    match element {
        InputElement::Keyboard { key } => if keycodes.pressed(*key) { 1.0 } else { 0.0 },
        InputElement::Mouse { button } => if mouse_buttons.pressed(*button) { 1.0 } else { 0.0 },
        InputElement::GamepadButton { button } => 
            if gamepad_buttons.pressed(GamepadButton { gamepad: Gamepad { id: 0 }, button_type: *button }) { 1.0 } else { 0.0 },
        InputElement::GamepadAxis { axis, mult } =>
            gamepad_axis.get(GamepadAxis { gamepad: Gamepad { id: 0 }, axis_type: *axis }).unwrap_or(0.) * *mult
    }
}

// events
#[derive(Event)]
pub struct InputPressedEvent { pub name: String, pub value: f32 }
#[derive(Event)]
pub struct InputDepressedEvent { pub name: String, pub value: f32 }
#[derive(Event)]
pub struct InputChangedEvent { pub name: String, pub value: f32 }
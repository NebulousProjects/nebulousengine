use bevy::{prelude::*, input::{mouse::MouseMotion, gamepad::{GamepadEvent, GamepadConnection}}, asset::{AssetLoader, LoadedAsset}, reflect::TypeUuid};
use enums::{mouse_button, gamepad_button_type, gamepad_axis_type};
use json::JsonValue;
use nebulousengine_utils::{load_file_to_json, optionals::{optional_string, optional_u32, optional_f32}};
use std::{collections::*};

use types::*;

pub mod types;
pub mod enums;

#[derive(Resource)]
pub struct GamepadContainer(Gamepad);

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // systems
            .add_system(gamepad_update)
            .add_system(update)

            // components and loaders
            .add_asset::<InputContainer>()
            .init_asset_loader::<InputContainerLoader>()

            // events
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
    // input stuff
    mut inputs: ResMut<Assets<InputContainer>>,
    mut pressed_events: EventWriter<InputPressedEvent>,
    mut released_events: EventWriter<InputReleasedEvent>,
    mut changed_events: EventWriter<InputChangedEvent>,

    // bevy input stuff
    windows: Query<&Window>,
    keys: Res<Input<ScanCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    pad_buttons: Res<Input<GamepadButton>>,
    pad_axes: Res<Axis<GamepadAxis>>,
    mut mouse_motion: EventReader<MouseMotion>,
    gamepad_container: Option<Res<GamepadContainer>>
) {
    let primary_window = windows.single();

    // what about handles?
    for (_, input) in inputs.iter_mut() {
        // loop through all active input rules
        for (name, value) in input.inputs.iter_mut() {
            // println!("Input name: {}", name);
            // get new and old values
            let old_value = value.value;
            let new_value = eval_value(&value, primary_window, &keys, &mouse_buttons, &pad_buttons, &pad_axes, &mut mouse_motion, &gamepad_container);

            // if the value has been pressed, broadcast event
            if old_value.abs() < value.press_threshold && new_value.abs() >= value.press_threshold {
                pressed_events.send(InputPressedEvent {
                    name: name.clone(),
                    value: new_value
                });
            }

            // if the value has been released, broadcast event
            if old_value.abs() >= value.press_threshold && new_value.abs() < value.press_threshold {
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
}

#[derive(Component, TypeUuid)]
#[uuid = "fa70fb4f-ea35-4186-8971-3191e888b7cc"]
pub struct InputContainer {
    inputs: HashMap<String, InputValue>,
    name: String,
    serverPath: String
}

#[derive(Default)]
pub struct InputContainerLoader;
impl AssetLoader for InputContainerLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // get json from bytes
            let str = std::str::from_utf8(bytes);
            if str.is_err() { return Err(bevy::asset::Error::msg("Could not convert bytes to json in input asset loader")) }
            let root_json = json::parse(str.unwrap());
            if root_json.is_err() { return Err(bevy::asset::Error::msg("Could not parse json in input asset loader")) }
            let root_json = root_json.unwrap();

            // root json must be array
            if root_json.is_array() {
                // create map and then assemble it by looping through all inputs in root json
                let mut map: HashMap<String, InputValue> = HashMap::new();
                println!("Length of root json is {}", root_json.len());
                for json_idx in 0 .. root_json.len() {
                    let json = &root_json[json_idx];
                    let (name, value) = get_from_json_object(json);
                    // println!("Created input value with name: {}", name.clone());
                    map.insert(name, value);
                }

                // unpack context
                let name = load_context.path().file_name().unwrap().to_str().unwrap().to_string();
                let path = load_context.path().to_str().unwrap().to_string();

                // assemble final object
                let container = InputContainer {
                    inputs: map,
                    name: name.clone(),
                    serverPath: path.clone()
                };

                // set default to final object
                load_context.set_default_asset(LoadedAsset::new(container));
                info!("Loaded inputs from {} at {}", name, path);

                // return ok
                Ok(())
            } else {
                Err(bevy::asset::Error::msg("Input json root was not array"))
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["input"]
    }
}

fn get_from_json_object(input: &JsonValue) -> (String, InputValue) {
    // get name
    let name = optional_string(input, "name").to_string();
    let press_threshold = optional_f32(input, "press_threshold", 1.0);

    // deocde description
    let mut descriptions = Vec::new();
    let descriptions_json = &input["descriptions"];
    if descriptions_json.is_array() {
        for i in 0 .. descriptions_json.len() {
            let result = decode_description(&descriptions_json[i]);
            if result.is_ok() {
                descriptions.push(result.unwrap());
            }
        }
    }

    // decode value
    let value = InputValue {
        press_threshold: press_threshold,
        value: 0.0,
        descriptions: descriptions
    };

    // save
    (name, value)
    // self.insert_or_update_input(name, value);
}

fn decode_description(input: &JsonValue) -> Result<InputDescription, String> {
    // get type
    let type_str = input["type"].as_str();
    if type_str.is_none() {
        return Err("Could not get type from input for input type decoding".to_string());
    }
    let type_str = type_str.unwrap();


    return match type_str {
        "scalar" => {
            let input_object = &input["input"];
            let input_enum = decode_input(input_object);
            if input_enum.is_ok() {
                Ok(InputDescription::Scalar { input_type: input_enum.unwrap() })
            } else {
                Err("Could not decode input".to_string())
            }
        },
        "axis" => {
            let positive_object = &input["positive_input"];
            let negative_object = &input["negative_input"];
            let positive_enum = decode_input(positive_object);
            let negative_enum = decode_input(negative_object);
            if positive_enum.is_ok() && negative_enum.is_ok() {
                Ok(InputDescription::Axis { positive_type: positive_enum.unwrap(), negative_type: negative_enum.unwrap() })
            } else {
                Err("Could not decode axis input".to_string())
            }
        },
        _ => Err(format!("Unknown type {} for decode description", type_str))
    }
}

fn decode_input(input: &JsonValue) -> Result<InputType, String> {
    // get type
    let type_str = input["type"].as_str();
    if type_str.is_none() {
        return Err("Could not get type from input for input type decoding".to_string());
    }
    let type_str = type_str.unwrap();

    return match type_str {
        "keyboard" => {
            let keycode_int = optional_u32(input, "keycode", 0);
            Ok(InputType::Keyboard(ScanCode(keycode_int)))
        },
        "mouse_motion_x" => Ok(InputType::MouseMotionX()),
        "mouse_motion_y" => Ok(InputType::MouseMotionY()),
        "mouse_button" => {
            let result = mouse_button(&input["button"]);
            if result.is_ok() {
                Ok(InputType::MouseButton(result.unwrap()))
            } else {
                Err(result.err().unwrap())
            }
        },
        "gamepad_button" => {
            let result = gamepad_button_type(&input["button"]);
            if result.is_ok() {
                Ok(InputType::GamepadButton(result.unwrap()))
            } else {
                Err(result.err().unwrap())
            }
        },
        "gamepad_axis" => {
            let result = gamepad_axis_type(&input["axis"]);
            if result.is_ok() {
                Ok(InputType::GamepadAxis(result.unwrap()))
            } else {
                Err(result.err().unwrap())
            }
        },
        _ => Err(format!("Input type \"{}\" unknown", type_str))
    }
}

// #[derive(Resource)]
// pub struct Inputs {
//     values: HashMap<String, InputValue>
// }
// impl Inputs {
//     pub fn insert_or_update_input(&mut self, name: String, value: InputValue) {
//         self.values.insert(name, value);
//     }

//     pub fn remove_input(&mut self, name: &String) {
//         self.values.remove(name);
//     }

//     pub fn clear(&mut self) {
//         self.values.clear();
//     }

//     pub fn get_value(&mut self, name: &String) -> Result<f32, String> {
//         let value = self.values.get(name);
//         if value.is_some() { Ok(value.unwrap().value) } 
//         else { Err(format!("Could not find input with name {}", name)) }
//     }

//     pub fn get_value_or_default(&mut self, name: &String, default: f32) -> f32 {
//         let output = self.get_value(name);
//         if output.is_ok() { output.unwrap() } else { default }
//     }

//     pub fn insert_from_path(&mut self, path: &str) {
//         let json = load_file_to_json(path);
//         if json.is_ok() {
//             self.insert_from_json_array(&json.unwrap());
//         } else {
//             error!("{}", json.err().unwrap())
//         }
//     }

//     pub fn insert_from_json_array(&mut self, input: &JsonValue) {
//         // make sure input is an array
//         if !input.is_array() {
//             error!("Input was not an array");
//             return;
//         }

//         // loop through all values
//         for i in 0 .. input.len() {
//             let json = &input[i];
//             self.insert_from_json_object(json);
//         }
//     }

//     pub fn insert_from_json_object(&mut self, input: &JsonValue) {
//         let (name, value) = self.get_from_json_object(input);
//         self.insert_or_update_input(name, value);
//     }

//     pub fn get_from_json_object(&mut self, input: &JsonValue) -> (String, InputValue) {
//         // get name
//         let name = optional_string(input, "name").to_string();
//         let press_threshold = optional_f32(input, "press_threshold", 1.0);

//         // deocde description
//         let mut descriptions = Vec::new();
//         let descriptions_json = &input["descriptions"];
//         if descriptions_json.is_array() {
//             for i in 0 .. descriptions_json.len() {
//                 let result = self.decode_description(&descriptions_json[i]);
//                 if result.is_ok() {
//                     descriptions.push(result.unwrap());
//                 }
//             }
//         }

//         // decode value
//         let value = InputValue {
//             press_threshold: press_threshold,
//             value: 0.0,
//             descriptions: descriptions
//         };

//         // save
//         (name, value)
//         // self.insert_or_update_input(name, value);
//     }

//     fn decode_description(&mut self, input: &JsonValue) -> Result<InputDescription, String> {
//         // get type
//         let type_str = input["type"].as_str();
//         if type_str.is_none() {
//             return Err("Could not get type from input for input type decoding".to_string());
//         }
//         let type_str = type_str.unwrap();


//         return match type_str {
//             "scalar" => {
//                 let input_object = &input["input"];
//                 let input_enum = self.decode_input(input_object);
//                 if input_enum.is_ok() {
//                     Ok(InputDescription::Scalar { input_type: input_enum.unwrap() })
//                 } else {
//                     Err("Could not decode input".to_string())
//                 }
//             },
//             "axis" => {
//                 let positive_object = &input["positive_input"];
//                 let negative_object = &input["negative_input"];
//                 let positive_enum = self.decode_input(positive_object);
//                 let negative_enum = self.decode_input(negative_object);
//                 if positive_enum.is_ok() && negative_enum.is_ok() {
//                     Ok(InputDescription::Axis { positive_type: positive_enum.unwrap(), negative_type: negative_enum.unwrap() })
//                 } else {
//                     Err("Could not decode axis input".to_string())
//                 }
//             },
//             _ => Err(format!("Unknown type {} for decode description", type_str))
//         }
//     }

//     fn decode_input(&mut self, input: &JsonValue) -> Result<InputType, String> {
//         // get type
//         let type_str = input["type"].as_str();
//         if type_str.is_none() {
//             return Err("Could not get type from input for input type decoding".to_string());
//         }
//         let type_str = type_str.unwrap();

//         return match type_str {
//             "keyboard" => {
//                 let keycode_int = optional_u32(input, "keycode", 0);
//                 Ok(InputType::Keyboard(ScanCode(keycode_int)))
//             },
//             "mouse_motion_x" => Ok(InputType::MouseMotionX()),
//             "mouse_motion_y" => Ok(InputType::MouseMotionY()),
//             "mouse_button" => {
//                 let result = mouse_button(&input["button"]);
//                 if result.is_ok() {
//                     Ok(InputType::MouseButton(result.unwrap()))
//                 } else {
//                     Err(result.err().unwrap())
//                 }
//             },
//             "gamepad_button" => {
//                 let result = gamepad_button_type(&input["button"]);
//                 if result.is_ok() {
//                     Ok(InputType::GamepadButton(result.unwrap()))
//                 } else {
//                     Err(result.err().unwrap())
//                 }
//             },
//             "gamepad_axis" => {
//                 let result = gamepad_axis_type(&input["axis"]);
//                 if result.is_ok() {
//                     Ok(InputType::GamepadAxis(result.unwrap()))
//                 } else {
//                     Err(result.err().unwrap())
//                 }
//             },
//             _ => Err(format!("Input type \"{}\" unknown", type_str))
//         }
//     }
// }
use bevy::{reflect::{TypeUuid, TypePath}, asset::{AssetLoader, LoadedAsset}, prelude::error};
use serde::*;

// asset loader for state machines
pub struct StateMachineLoader;
impl AssetLoader for StateMachineLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // load content
            let content = std::str::from_utf8(bytes);
            if content.is_err() { error!("Failed to load state machine json!"); return Err(bevy::asset::Error::msg("Failed to load json for state machine")) }
            let content = content.unwrap();
            
            // load description
            let machine: Result<StateMachine, serde_json::Error> = 
                serde_json::from_str(content);
            if machine.is_err() { error!("Failed to load state machine from json with error: {}", machine.err().unwrap()); return Err(bevy::asset::Error::msg("Failed to load struct for state machine")) }
            
            // load final input map
            load_context.set_default_asset(LoadedAsset::new(machine.unwrap()));
            
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["statemachine"]
    }
}

// struct representing a state machine
#[derive(TypeUuid, TypePath, Serialize, Deserialize)]
#[uuid = "cdd4fda9-ae74-4589-bfd0-267530f5a57e"]
pub struct StateMachine {
    pub default: String,
    pub elements: Vec<StateMachineElement>,
}

// struct for each element of a state machine
#[derive(Serialize, Deserialize)]
pub struct StateMachineElement {
    pub name: String,
    pub next: Vec<StateMachineNext>,
    pub triggers: Vec<StateMachineTrigger>
}

#[derive(Serialize, Deserialize)]
pub struct StateMachineTrigger {
    pub trigger: String,
    pub priority: f32
}

// the next options of an element
#[derive(Serialize, Deserialize)]
pub struct StateMachineNext {
    pub name: String,
    pub condition: StateMachineCondition,
    pub priority: f32
}

// state machine conditions
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StateMachineCondition {
    None
}


use bevy::{reflect::{TypeUuid, TypePath}, asset::{AssetLoader, AsyncReadExt}, prelude::*};
use serde::*;

#[derive(Debug)]
pub struct StateMachineLoadError(String);

impl std::fmt::Display for StateMachineLoadError {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StateMachineLoadError({})", self.0)
    }
}

impl std::error::Error for StateMachineLoadError {}

// asset loader for state machines
#[derive(Default)]
pub struct StateMachineLoader;
impl AssetLoader for StateMachineLoader {
    type Asset = StateMachine;
    type Error = StateMachineLoadError;
    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // load content
            let mut bytes = Vec::new();
            let error = reader.read_to_end(&mut bytes).await;
            if error.is_err() { return Err(StateMachineLoadError("Failed to load text bytes!".into())) }
            let content = std::str::from_utf8(&bytes);
            if content.is_err() { error!("Failed to load state machine json!"); return Err(StateMachineLoadError("Failed to load json for state machine".into())) }
            let content = content.unwrap();
            
            // load description
            let machine: Result<StateMachine, serde_json::Error> = 
                serde_json::from_str(content);
            if machine.is_err() { error!("Failed to load state machine from json with error: {}", machine.err().unwrap()); return Err(StateMachineLoadError("Failed to load struct for state machine".into())) }
            
            // load final input map
            // load_context.set_default_asset(LoadedAsset::new());
            
            Ok(machine.unwrap())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["statemachine"]
    }
}

// struct representing a state machine
#[derive(TypeUuid, TypePath, Serialize, Deserialize, Asset)]
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


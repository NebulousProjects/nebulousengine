use bevy::{prelude::*, utils::HashMap};

use nebulousengine_utils::state_machines::{asset::StateMachine, StateMachineContext};

// component for animated objects
#[derive(Component, Default)]
pub struct Animator {
    pub handle: Handle<StateMachine>,
    pub current_animation: Option<String>,
    pub triggers: Vec<String>,
    pub values: HashMap<String, f32>
}

// add a state machine context to the animator
impl StateMachineContext for Animator {
    fn get_current_triggers(&self) -> &Vec<String> {
        &self.triggers
    }

    fn get_current_values(&self) -> &HashMap<String, f32> {
        &self.values
    }

    fn get_set_state(&self) -> &Option<String> {
        &self.current_animation
    }
}

// add control functions to animator
impl Animator {
    // called to trigger an event in the animator
    pub fn trigger(&mut self, trigger: String) {
        self.triggers.push(trigger);
    }

    // called to set a value
    pub fn set_value(&mut self, name: String, value: f32) {
        self.values.insert(name, value);
    }

    // get a value
    pub fn get_value(&self, name: &String) -> Option<&f32> {
        self.values.get(name)
    }
}
use bevy::{prelude::*, utils::HashMap};

use self::asset::{StateMachine, StateMachineLoader, StateMachineCondition};

pub mod asset;

// a context that allows the same state machine to be used in multiple different areas
pub trait StateMachineContext {
    fn get_current_triggers(&self) -> &Vec<String>;
    fn get_current_values(&self) -> &HashMap<String, f32>;
    fn get_set_state(&self) -> &Option<String>;
}

// plugin for state machines
pub struct StateMachinePlugin;
impl Plugin for StateMachinePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<StateMachine>()
            .add_asset_loader(StateMachineLoader);
    }
}

#[derive(Clone, Debug)]
pub struct StateMachineResult {
    pub next: String,
    pub triggered: Option<String>
}

// add a functions to get the current state of a state machine with a context
impl StateMachine {
    pub fn current_state(&self, context: &impl StateMachineContext) -> StateMachineResult {
        // get set state
        let set_state = context.get_set_state();
        let set_state = if set_state.is_some() { set_state.clone().unwrap() } else { self.default.clone() };

        // setup some variables
        let current_element = self.elements.iter().filter(|element| element.name == set_state).next();

        // look for any elements with triggers
        let active_triggers = context.get_current_triggers();
        let mut triggered: Option<String> = None;
        let mut triggered_prio = -1.;
        self.elements.iter().for_each(|element| {
            let trigger = element.triggers.iter()
                .filter(|trigger| { active_triggers.contains(&trigger.trigger) && trigger.priority > triggered_prio })
                .next();
            if trigger.is_some() {
                let trigger = trigger.unwrap();
                triggered = Some(element.name.clone());
                triggered_prio = trigger.priority;
            }
        });

        // get the current next state
        let next = if current_element.is_some() {
            let current_element = current_element.unwrap();

            // look for next with higher priority that currently set
            let mut next_state: Option<String> = None;
            let mut next_prio = -1.;
            current_element.next.iter().for_each(|next| {
                if next.priority > next_prio && eval_condition_for_context(context, &next.condition) {
                    next_state = Some(next.name.clone());
                    next_prio = next.priority;
                }
            });

            // return next state if one was found, otherwise, default
            if next_state.is_some() { next_state.unwrap() } else { self.default.clone() }
        } else { self.default.clone() }; // if current element was not found, return default

        // pass back result
        StateMachineResult { next, triggered }
    }
}

// take a state machine condition and context and evaluates if the context meets the condition
pub fn eval_condition_for_context(_: &impl StateMachineContext, condition: &StateMachineCondition) -> bool {
    match condition {
        StateMachineCondition::None => true,
    }
}
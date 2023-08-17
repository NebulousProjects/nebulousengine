use bevy::prelude::*;
use nebulousengine_utils::state_machines::asset::StateMachine;

use crate::gltf::GLTFModel;

use self::component::Animator;

pub mod component;

pub struct AnimatorPlugin;
impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animated_gltfs);
    }
}

// System that updates all active animators with gltf models
fn update_animated_gltfs(
    mut query: Query<(&mut GLTFModel, &mut Animator)>,
    machines: Res<Assets<StateMachine>>
) {
    // loop through all entities in the query
    query.for_each_mut(|(mut model, mut animator)| {
        // attempt to get state machine of animator
        let machine = machines.get(&animator.handle);
        if machine.is_none() { return }
        let machine = machine.unwrap();

        // get current state from machine
        let state = machine.current_state(animator.as_ref());

        // clear triggers
        animator.triggers.clear();

        // update next animation
        model.set_next_animation(state.next);

        // if triggered animation is something, start that animation and stop here
        if state.triggered.is_some() {
            model.set_current_animation(state.triggered.unwrap());
        }

        // make sure current animation is up to date
        animator.current_animation = model.get_current_animation().clone();
    })
}
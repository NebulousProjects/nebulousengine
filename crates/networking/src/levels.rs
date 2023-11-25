use std::{marker::PhantomData, hash::Hash, fmt::Debug};

use bevy::prelude::*;
use nebulousengine_levels::{*, levels::*};
use serde::{*, de::DeserializeOwned};

use crate::{NetworkStateReceiveEvent, Networking};

// create plugin for a network state
#[derive(Default)]
pub struct SyncedLevel<T: States + Default + Debug + Eq + PartialEq + Hash + Serialize + DeserializeOwned + 'static>(pub PhantomData<T>);
impl <T: States + Default + Debug + Eq + PartialEq + Hash + Serialize + DeserializeOwned + 'static> Plugin for SyncedLevel<T> {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Levels::<T>::default())
            .add_systems(Update, control_state::<T>);
    }
}

// system for controlling states
fn control_state<T: States + Default + Debug + Eq + PartialEq + Hash + Serialize + DeserializeOwned + 'static>(
    mut net: ResMut<Networking>,
    mut level: ResMut<Level<T>>,
    mut recv_states: EventReader<NetworkStateReceiveEvent>,
    current_state: Res<State<T>>,
) {
    // if server and state changed, broadcast set states and update own state
    if net.is_server() && current_state.is_changed() {
        net.send(crate::NetworkPacketWrapper::SetState { state: serde_json::to_value(current_state.get().clone()).unwrap() });
    }

    // if client, update state based on recv states
    if net.is_client() && !recv_states.is_empty() {
        // get next state
        let new_state = recv_states.read().next();
        let new_state = if new_state.is_some() { new_state.unwrap() } else { return };

        // deserialize new state
        let new_state = serde_json::from_value::<T>(new_state.0.clone()).unwrap();

        // change state
        level.goto(new_state);
    }
}

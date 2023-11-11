use std::marker::PhantomData;

use bevy::prelude::*;
use serde::{*, de::DeserializeOwned};

use crate::{NetworkStateReceiveEvent, Networking};

// events for state control
#[derive(Event)]
pub struct SetState<T: NetworkedState>(pub T);

// trait to mark a network state
pub trait NetworkedState: States + Serialize + DeserializeOwned + 'static {}

// create plugin for a network state
#[derive(Default)]
pub struct NetworkedStateController<T: NetworkedState>(PhantomData<T>);
impl <T: NetworkedState> Plugin for NetworkedStateController<T> {
    fn build(&self, app: &mut App) {
        app
            .add_state::<T>()
            .add_event::<SetState<T>>()
            .add_systems(Update, control_state::<T>);
    }
}

// system for controlling states
fn control_state<T: NetworkedState>(
    mut net: ResMut<Networking>,
    current_state: Res<State<T>>,
    mut state: ResMut<NextState<T>>,
    mut recv_states: EventReader<NetworkStateReceiveEvent>,
    mut set_states: EventReader<SetState<T>>
) {
    // if server, broadcast set states and update own state
    if net.is_server() && !set_states.is_empty() {
        // get next state
        let new_state = set_states.iter().next();
        let new_state = if new_state.is_some() { new_state.unwrap() } else { return };

        // change state
        state.set(new_state.0.clone());

        // broadcast new state
        net.send(crate::NetworkPacketWrapper::SetState { state: serde_json::to_value(new_state.0.clone()).unwrap() });
    }

    // if client, update state based on recv states
    if net.is_client() && !recv_states.is_empty() {
        // get next state
        let new_state = recv_states.iter().next();
        let new_state = if new_state.is_some() { new_state.unwrap() } else { return };

        // deserialize new state
        let new_state = serde_json::from_value::<T>(new_state.0.clone()).unwrap();

        // change state
        if new_state != *current_state.get() { state.set(new_state); }
    }
}

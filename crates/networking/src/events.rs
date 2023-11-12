use std::{net::SocketAddr, fmt::Debug};

use __private::PhantomData;
use bevy::{prelude::*, reflect::TypePath};
use serde::{*, de::DeserializeOwned};
use serde_json::*;

use crate::Networking;

// plugin for adding networked event
#[derive(Debug, Default)]
pub struct GameNetworkSyncedEvent<T: NetworkedEvent>(PhantomData<T>);
impl<T: NetworkedEvent> Plugin for GameNetworkSyncedEvent<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<T>().add_systems(Update, (push_event::<T>, pull_event::<T>));
    }
}

// setup event and its wrapper
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkEventWrapper {
    pub event: String,
    pub value: Value,
    pub from_addr: Option<SocketAddr>
}

// network event
pub trait NetworkedEvent: Event + Serialize + DeserializeOwned + Debug + TypePath + Default {
    fn set_received(&mut self, received: bool);
    fn get_received(&self) -> bool;
}

// system for pushing events to the network
fn push_event<T: NetworkedEvent>(
    mut net: ResMut<Networking>,
    mut reader: EventReader<T>
) {
    // read incoming events
    reader.read().for_each(|event| {
        if event.get_received() { return }

        // serialize event to value
        let value = serde_json::to_value(event);
        let value = if value.is_ok() { value.unwrap() } else { error!("Failed to convert event {:?} to value!", event); return };
        let value = NetworkEventWrapper { event: T::type_path().into(), value, from_addr: None };

        // send value
        net.send(crate::NetworkPacketWrapper::Event(value));
    });
}

// system for pulling events from the network
fn pull_event<T: NetworkedEvent>(
    mut net: ResMut<Networking>,
    mut writer: EventWriter<T>
) {
    // create some temp variables
    let is_server = net.is_server();
    let mut broadcast: Vec<(NetworkEventWrapper, SocketAddr)> = Vec::new();

    // broadcast incoming events
    net.waiting_events.retain(|wrapper| {
        if wrapper.event == T::type_path() {
            let event = serde_json::from_value::<T>(wrapper.value.clone());
            if event.is_ok() {
                // unpack event
                let mut event = event.unwrap();
                event.set_received(true);

                // write event
                writer.send(event);

                // if net is server, broadcast ignoring the wrappers from addr
                if is_server && wrapper.from_addr.is_some() {
                    broadcast.push((wrapper.clone(), wrapper.from_addr.unwrap()));
                }
            } else {
                error!("Failed to decode event {} with error: {}", T::type_path(), event.err().unwrap());
            }
            false
        } else { true }
    });

    // handle broadcast vector
    broadcast.iter().for_each(|(wrapper, addr)| {
        net.broadcast_ignorable(*addr, crate::NetworkPacketWrapper::Event(wrapper.clone()));
    });
}

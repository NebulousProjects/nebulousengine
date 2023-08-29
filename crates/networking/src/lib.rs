use std::net::{TcpStream, TcpListener, SocketAddr};

use bevy::{prelude::*, utils::HashMap};
use events::NetworkEventWrapper;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use structs::{NetworkID, NetworkTransform, NetworkBuilderInfo, NetworkTransformTracker};
use tungstenite::{connect, WebSocket, stream::MaybeTlsStream, accept, Message};
use url::Url;

pub mod builder;
pub mod events;
pub mod states;
pub mod structs;

#[derive(Resource, Debug, Default)]
pub struct Networking {
    pub server: Option<TcpListener>,
    pub connections: Vec<WebSocket<TcpStream>>,
    pub client: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    pub waiting_events: Vec<NetworkEventWrapper>,
    pub event_tickers: HashMap<String, usize>,
    pub my_id: u8,
    pub id_tracker: u8
}

#[derive(Serialize, Deserialize)]
pub enum NetworkPacketWrapper {
    Event(NetworkEventWrapper),
    SetNetID(u8),
    SpawnEntity { build: Value, transform: NetworkTransform, network_id: NetworkID },
    RemoveEntity { network_id: NetworkID },
    UpdateEntityTransform { network_id: NetworkID, transform: NetworkTransform, velocity: Vec3 },
    SetState { state: Value }
}

// event for new connection
#[derive(Event, Debug)]
pub struct NetworkServerNewConnectionEvent(pub u8);

// create events so that networking can call spawns without knowning the builder generic type
#[derive(Event, Debug)]
pub struct NetworkCallSpawnEvent { pub build: Value, pub transform: NetworkTransform, pub network_id: NetworkID }

#[derive(Event, Debug)]
pub struct NetworkCallDespawnEvent { pub network_id: NetworkID }

#[derive(Event, Debug)]
pub struct NetworkCallUpdateTransformEvent { pub network_id: NetworkID, pub transform: NetworkTransform, pub velocity: Vec3 }

#[derive(Event, Debug)]
pub struct NetworkStateReceiveEvent(Value);

impl Networking {
    pub fn established(&self) -> bool { self.server.is_some() || self.client.is_some() }
    pub fn is_server(&self) -> bool { self.server.is_some() }
    pub fn is_client(&self) -> bool { self.client.is_some() }

    pub fn send(&mut self, packet: NetworkPacketWrapper) {
        // create message
        let message = serde_json::to_string(&packet).unwrap();

        if self.is_server() {
            // send to each connection
            self.connections.iter_mut().for_each(|a| {
                let send_result = a.send(Message::Text(message.clone()));
                if send_result.is_err() { error!("Failed server broadcast send {}", send_result.err().unwrap()); }
            });
        } else {
            let client = self.client.as_mut().unwrap();
            let send_result = client.send(Message::Text(message));
            if send_result.is_err() { error!("Failed client send {}", send_result.err().unwrap()); }
        }
    }

    // todo function that sends to all except a specific client
    pub fn broadcast_ignorable(&mut self, ignore_addr: SocketAddr, packet: NetworkPacketWrapper) {
        // create message
        let message = serde_json::to_string(&packet).unwrap();

        if self.is_server() {
            // send to each connection
            self.connections.iter_mut().for_each(|socket| {
                // skip if ignored
                if socket.get_ref().peer_addr().unwrap() == ignore_addr { return }

                // send message
                let send_result = socket.send(Message::Text(message.clone()));
                if send_result.is_err() { error!("Failed server broadcast send {}", send_result.err().unwrap()); }
            });
        } else {
            let client = self.client.as_mut().unwrap();
            let send_result = client.send(Message::Text(message));
            if send_result.is_err() { error!("Failed client send {}", send_result.err().unwrap()); }
        }
    }
}

pub struct GameNetworkingPlugin;
impl Plugin for GameNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NetworkServerNewConnectionEvent>()
            .add_event::<NetworkCallSpawnEvent>()
            .add_event::<NetworkCallDespawnEvent>()
            .add_event::<NetworkCallUpdateTransformEvent>()
            .add_event::<NetworkStateReceiveEvent>()
            .insert_resource(Networking::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (accept_connections, recv_packets, update_transforms));
    }
}

fn setup(mut net: ResMut<Networking>) {
    let client = connect(Url::parse("ws://127.0.0.1:8050").unwrap());

    // if a connection was made, save it, otherwise, create the server
    if client.is_ok() {
        let mut client = client.unwrap().0;

        // make stream non blocking
        let stream = client.get_mut();
        match stream {
            MaybeTlsStream::Plain(stream) => {
                let nonblocking = stream.set_nonblocking(true);
                if nonblocking.is_err() { error!("Set non blocking failed with error: {}", nonblocking.err().unwrap()); }
            },
            _ => error!("Unknown maybe tls stream type: {:?}", stream),
        }

        // save client
        net.client = Some(client);
        info!("Client connected to server!");
    } else {
        // create server
        let server = TcpListener::bind("127.0.0.1:8050").unwrap();
        let nonblocking = server.set_nonblocking(true);
        net.server = Some(server);
        info!("Created server!");

        if nonblocking.is_err() { error!("Set non blocking failed with error: {}", nonblocking.err().unwrap()); }
    }
}

fn accept_connections(
    mut net: ResMut<Networking>, 
    networked_entities: Query<(&NetworkBuilderInfo, &Transform, &NetworkID)>,
    mut connect_events: EventWriter<NetworkServerNewConnectionEvent>
) {
    // accept incoming connections
    if net.is_server() {
        // unpack net
        let server = net.server.as_mut().unwrap();
        let mut new_connections = Vec::new();
        
        // for each incoming connections
        for stream in server.incoming() {
            match stream {
                Ok(stream) => {
                    // accept connection
                    println!("Accepting connection {:?}", stream.peer_addr());
                    let socket = accept(stream);
                    match socket {
                        Ok(socket) => {
                            new_connections.push(socket);
                        },
                        Err(error) => error!("Accepting connection failed with error: {}", error)
                    };
                },
                Err(_) => break // error!("Error in server incoming tcp stream: {}", error), // todo maybe there is a way to ignore events from being nonblocking
            }
        }

        // send entities to new connections
        new_connections.iter_mut().for_each(|connection| {
            // set net id
            net.id_tracker += 1;
            let wrapper = NetworkPacketWrapper::SetNetID(net.id_tracker);
            let send_result = connection.send(Message::Text(serde_json::to_string(&wrapper).unwrap()));
            if send_result.is_err() { error!("Error in new connection net ID update: {:?}", send_result); }
            connect_events.send(NetworkServerNewConnectionEvent(net.id_tracker));

            // send networked entities
            networked_entities.for_each(|(info, transform, id)| {
                let wrapper = NetworkPacketWrapper::SpawnEntity { build: info.0.clone(), transform: NetworkTransform::from_bevy(transform), network_id: *id };
                let send_result = connection.send(Message::Text(serde_json::to_string(&wrapper).unwrap()));
                if send_result.is_err() { error!("Error in new connection exist entities update: {:?}", send_result); }
            });
        });

        // update connections
        net.connections.extend(new_connections);
    }
}

fn recv_packets(
    mut net: ResMut<Networking>, 
    mut spawn_events: EventWriter<NetworkCallSpawnEvent>, 
    mut despawn_events: EventWriter<NetworkCallDespawnEvent>,
    mut transform_events: EventWriter<NetworkCallUpdateTransformEvent>,
    mut state_events: EventWriter<NetworkStateReceiveEvent>,
) {
    // objects for updating net
    let mut new_waiting_events = Vec::new();
    let mut my_id = 0 as u8;

    // receive from connections
    if net.is_server() {
        net.connections.iter_mut().for_each(|stream| {
            // read all waiting
            while let Ok(read) = stream.read() {
                // read message
                process_incoming(
                    read, &mut new_waiting_events, 
                    &mut spawn_events, &mut despawn_events, &mut transform_events, &mut state_events,
                    &mut my_id, stream.get_ref().peer_addr().unwrap()
                );
            }
        });
    } else {
        // read messages from client
        let client = net.client.as_mut().unwrap();
        while let Ok(read) = client.read() {
            let addr = match client.get_ref() {
                MaybeTlsStream::Plain(client) => client.peer_addr().unwrap(),
                _ => todo!(),
            };

            // read message
            process_incoming(
                read, &mut new_waiting_events, 
                &mut spawn_events, &mut despawn_events, &mut transform_events, &mut state_events,
                &mut my_id, addr
            );
        }
    }

    // update net
    net.waiting_events.extend(new_waiting_events);
    if my_id != 0 { net.my_id = my_id; }
}

fn process_incoming(
    message: Message, 
    waiting_events: &mut Vec<NetworkEventWrapper>, 
    spawn_events: &mut EventWriter<NetworkCallSpawnEvent>, 
    despawn_events: &mut EventWriter<NetworkCallDespawnEvent>,
    transform_events: &mut EventWriter<NetworkCallUpdateTransformEvent>,
    state_events: &mut EventWriter<NetworkStateReceiveEvent>,
    my_id: &mut u8,
    addr: SocketAddr
) {
    // match to message type
    match message {
        Message::Text(text) => {
            // unpack packet wrapper
            let wrapper = serde_json::from_str::<NetworkPacketWrapper>(text.as_str());
            if wrapper.is_ok() {
                let wrapper = wrapper.unwrap();

                // match wrapper to create function
                match wrapper {
                    // events
                    NetworkPacketWrapper::Event(mut wrapper) => {
                        wrapper.from_addr = Some(addr);
                        waiting_events.push(wrapper);
                    },

                    NetworkPacketWrapper::SetNetID(id) => { *my_id = id; },

                    // entity control
                    NetworkPacketWrapper::SpawnEntity { build, transform, network_id } => {
                        spawn_events.send(NetworkCallSpawnEvent { build, transform, network_id });
                    },

                    NetworkPacketWrapper::RemoveEntity { network_id } => {
                        despawn_events.send(NetworkCallDespawnEvent { network_id });
                    },

                    NetworkPacketWrapper::UpdateEntityTransform { network_id, transform, velocity } => {
                        transform_events.send(NetworkCallUpdateTransformEvent { network_id, transform, velocity });
                    },

                    NetworkPacketWrapper::SetState { state } => {
                        state_events.send(NetworkStateReceiveEvent(state))
                    }
                }
            } else {
                error!("Failed to unpack input text to wrapper with error: {}", wrapper.err().unwrap());
            }
        },

        Message::Binary(_) => todo!(),
        Message::Ping(_) => todo!(),
        Message::Pong(_) => todo!(),
        Message::Close(_) => todo!(),
        Message::Frame(_) => todo!(),
    }
}

fn update_transforms(
    mut net: ResMut<Networking>,
    mut updated_network_entities: Query<(&NetworkID, &mut NetworkTransformTracker, &mut Transform)>,
    time: Res<Time>
) {
    updated_network_entities.for_each_mut(|(id, mut tracker, mut transform)| {
        // if owned, process transform update
        if net.my_id == id.owner {
            // make sure transform changed
            let velocity = (transform.translation - tracker.last_transform.translation) / time.delta_seconds();
            if tracker.last_transform == *transform && tracker.velocity == velocity { return }
            tracker.last_transform = *transform;
            tracker.velocity = velocity;

            // send transform update
            net.send(NetworkPacketWrapper::UpdateEntityTransform { network_id: *id, transform: NetworkTransform::from_bevy(&transform), velocity });
        }
        // otherwise, update velocity
        else {
            transform.translation += tracker.velocity * time.delta_seconds();
        }
    });
}

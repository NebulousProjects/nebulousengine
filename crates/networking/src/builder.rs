use std::fmt::Debug;

use bevy::{prelude::*, ecs::system::EntityCommands};
use serde::{*, de::DeserializeOwned};

use crate::{Networking, structs::{NetworkID, NetworkTransform, NetworkBuilderInfo, NetworkTransformTracker}, NetworkCallDespawnEvent, NetworkCallSpawnEvent, NetworkCallUpdateTransformEvent};

// create plugin to make the process of creating entity builder easy
pub struct GameNetworkEntityBuilder<T: Serialize + DeserializeOwned + Debug + Clone + Event> {
    create_callback: fn(commands: EntityCommands, build_type: T, is_owned: bool)
}

impl<T: Serialize + DeserializeOwned + Debug + Clone + Event> GameNetworkEntityBuilder<T> {
    pub fn new(create_callback: fn(commands: EntityCommands, build_type: T, is_owned: bool)) -> Self {
        Self { create_callback }
    }
}

// implement builder and its systems as a plugin
impl<T: Serialize + DeserializeOwned + Debug + Clone + Event> Plugin for GameNetworkEntityBuilder<T> {
    fn build(&self, app: &mut App) {
        app
            .insert_non_send_resource(NetworkEntityBuilder::<T>::new(self.create_callback))
            .add_systems(Update, handle_builder_queues::<T>);
    }
}

// struct to contain builder
#[derive(Clone, Debug)]
pub struct NetworkEntityBuilder<T: Serialize + DeserializeOwned + Debug + Clone + Event> {
    pub(crate) create_callback: fn(commands: EntityCommands, build_type: T, is_owned: bool),
    pub(crate) queued_builds: Vec<(T, Transform, NetworkID)>,
    pub(crate) queued_despawn: Vec<NetworkID>,
    pub(crate) cur_network_id: u64
}

// add create and modify functions
impl<T: Serialize + DeserializeOwned + Debug + Clone + Event> NetworkEntityBuilder<T> {
    pub fn new(callback: fn(commands: EntityCommands, build_type: T, owned: bool)) -> Self {
        Self {
            create_callback: callback,
            queued_builds: Vec::default(),
            queued_despawn: Vec::default(),
            cur_network_id: 0
        }
    }

    pub fn queue_build(&mut self, build: T, transform: Transform, id: NetworkID) { self.queued_builds.push((build, transform, id)); }
    pub fn queue_despawn(&mut self, network_id: NetworkID) { self.queued_despawn.push(network_id); }

    
    pub fn queue_build_owned(&mut self, build: T, transform: Transform, owner: u8) {
        let net_id = self.cur_network_id;
        self.cur_network_id += 1;
        let net_id = NetworkID { net_id, owner };
        self.queued_builds.push((build, transform, net_id));
    }
}

// setup all necessary systems
pub fn handle_builder_queues<T: Serialize + DeserializeOwned + Debug + Clone + Event>(
    mut commands: Commands,
    mut net: ResMut<Networking>,
    mut builder: NonSendMut<NetworkEntityBuilder<T>>,
    mut net_query: Query<(Entity, &NetworkID, &mut Transform, &mut NetworkTransformTracker)>,
    mut spawn_events: EventReader<NetworkCallSpawnEvent>,
    mut despawn_events: EventReader<NetworkCallDespawnEvent>,
    mut transform_events: EventReader<NetworkCallUpdateTransformEvent>
) {
    // handle spawn events
    spawn_events.iter().for_each(|event| {
        builder.queue_build(serde_json::from_value(event.build.clone()).unwrap(), event.transform.to_bevy(), event.network_id);
    });

    // handle transform updates
    transform_events.iter().for_each(|event| {
        let to_move = net_query.iter_mut().find(|(_, a, _, _)| a.net_id == event.network_id.net_id);
        if to_move.is_some() {
            let (_, _, mut transform, mut tracker) = to_move.unwrap();
            transform.translation = event.transform.translation;
            transform.rotation = event.transform.rotation;
            transform.scale = event.transform.scale;
            tracker.velocity = event.velocity;
        }
    });

    // handle despawn events
    despawn_events.iter().for_each(|event| {
        builder.queue_despawn(event.network_id);
    });

    // run all queued builds
    builder.queued_builds.iter().for_each(|(build, transform, owner)| {
        // spawn entity
        let entity = commands.spawn((
            VisibilityBundle::default(),
            TransformBundle {
                local: *transform,
                ..Default::default()
            }, 
            owner.clone(), 
            NetworkBuilderInfo(serde_json::to_value(build).unwrap()),
            NetworkTransformTracker { last_transform: *transform, velocity: Vec3::ZERO }
        ));

        // build entity
        let is_owned = net.my_id == (*owner).owner;
        (builder.create_callback)(entity, build.clone(), is_owned);

        // broadcast if server
        if net.is_server() {
            net.send(crate::NetworkPacketWrapper::SpawnEntity { 
                build: serde_json::to_value(build.clone()).unwrap(), 
                transform: NetworkTransform::from_bevy(transform), 
                network_id: *owner
            });
        }
    });
    builder.queued_builds.clear();

    // run all queued despawns
    builder.queued_despawn.iter().for_each(|network_id| {
        // despawn with network ID
        let to_despawn = net_query.iter().find(|(_, a, _, _)| a.net_id == network_id.net_id);
        if to_despawn.is_some() {
            commands.entity(to_despawn.unwrap().0).despawn_recursive();
        }

        // broadcast if server
        if net.is_server() {
            net.send(crate::NetworkPacketWrapper::RemoveEntity { network_id: *network_id });
        }
    });
    builder.queued_despawn.clear();
}

// pub fn send_
use std::fmt::Debug;

use bevy::prelude::*;
use serde::*;
use serde_json::Value;

// a network ID
#[derive(Component, Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize, Default)]
pub struct NetworkID {
    pub owner: u8,
    pub net_id: u64
}

#[derive(Component)]
pub struct NetworkTransformTracker {
    pub last_transform: Transform,
    pub velocity: Vec3
}

// custom transform to represent bevy transform with serialize and deserialize
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NetworkTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3
}

// functions to convert to and from bevy transform
impl NetworkTransform {
    pub fn from_bevy(transform: &Transform) -> Self {
        Self {
            translation: transform.translation,
            rotation: transform.rotation,
            scale: transform.scale
        }
    }

    pub fn to_bevy(&self) -> Transform {
        Transform {
            translation: self.translation, 
            rotation: self.rotation, 
            scale: self.scale
        }
    }
}

// component for storing entity builder
#[derive(Component, Debug, Serialize, Deserialize)]
pub struct NetworkBuilderInfo(pub Value);

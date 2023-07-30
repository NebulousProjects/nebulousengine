use animator::AnimatorPlugin;
use bevy::prelude::*;
use gltf::GLTFPlugin;

pub mod gltf;
pub mod animator;

pub struct ModelPlugins;
impl Plugin for ModelPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((AnimatorPlugin, GLTFPlugin));
    }
}
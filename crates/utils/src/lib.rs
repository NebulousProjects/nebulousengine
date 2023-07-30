use bevy::prelude::*;
use follow_camera::FollowCameraPlugin;
use state_machines::StateMachinePlugin;

pub mod state_machines;
pub mod follow_camera;

pub struct NebulousEngineUtils;
impl Plugin for NebulousEngineUtils {
    fn build(&self, app: &mut App) {
        app.add_plugins((FollowCameraPlugin, StateMachinePlugin));
    }
}
use bevy::prelude::*;

#[derive(Component)]
pub struct FollowCamera {
    pub offset: Vec3,
    pub look_offset: Vec3
}
#[derive(Component)]
pub struct FollowCameraTarget {
    pub snap_to: bool
}

impl Default for FollowCameraTarget {
    fn default() -> Self {
        Self { snap_to: false }
    }
}

pub struct FollowCameraPlugin;
impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_to_target);
    }
}

// when a new target is added, snap to it if enabled
fn move_to_target(
    mut camera: Query<(&mut Transform, &FollowCamera)>,
    target: Query<(&Transform, &FollowCameraTarget, Without<FollowCamera>)>
) {
    // get single camera
    let camera = camera.get_single_mut();
    if camera.is_err() { return }
    let (mut camera, camera_settings) = camera.unwrap();

    // get target
    let target = target.get_single();
    if target.is_err() { return }
    let (target_transform, target, _) = target.unwrap();

    // update translation based on snap to target
    if target.snap_to {
        camera.translation = target_transform.translation + camera_settings.offset;
    } else {
        camera.translation = camera.translation.lerp(target_transform.translation + camera_settings.offset, 0.1);
    }

    // look at target FIXME currently causes weird camera rotations with above lerp
    // camera.look_at(target_transform.translation + camera_settings.look_offset, Vec3::Y);
}

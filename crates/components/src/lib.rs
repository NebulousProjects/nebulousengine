use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct ViewportContainer {
    pub image_handle: Option<Handle<Image>>,
    pub size: Extent3d,
    pub setup: bool
}

impl Default for ViewportContainer {
    fn default() -> Self {
        let size = Extent3d {
            width: 200,
            height: 200,
            ..default()
        };
        ViewportContainer {
            image_handle: None,
            size: size,
            setup: false
        }
    }
}
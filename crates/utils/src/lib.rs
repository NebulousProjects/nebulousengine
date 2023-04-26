use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use json::{ JsonValue };

pub mod optionals;
pub mod enums;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Despawnable;

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

pub fn load_file_to_json(path: &str) -> JsonValue {
    let file_contents = std::fs::read_to_string(path).unwrap();
    return json::parse(file_contents.as_str()).unwrap();
}
use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use json::{ JsonValue };

pub mod optionals;
pub mod enums;
pub mod from_enums;

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

pub fn load_file_to_json(path: &str) -> Result<JsonValue, String> {
    let file_contents = std::fs::read_to_string(format!("./assets/{}", path));

    if file_contents.is_ok() {
        let json = json::parse(file_contents.unwrap().as_str());
        if json.is_ok() {
            Ok(json.unwrap())
        } else {
            Err(format!("Json parse failed with error: {}", json.err().unwrap()))
        }
        // json::parse(file_contents.as_str()).unwrap()
    } else {
        Err(format!("Get file contents failed with error: {}", file_contents.err().unwrap()))
    }
}

pub fn insert_json(json: &mut JsonValue, name: &str, value: JsonValue) {
    let result = json.insert(name, value);
    if result.is_err() {
        error!("JSON insert failed with error: {}", result.err().unwrap());
    }
}

#[derive(Resource)]
pub struct RunningState {
    pub running: bool
}

impl Default for RunningState {
    fn default() -> Self {
        Self { running: true }
    }
}

#[macro_export]
macro_rules! is_of_var {
    ($val:ident, $var:path) => {
        match $val {
            $var{..} => true,
            _ => false
        }
    }
}
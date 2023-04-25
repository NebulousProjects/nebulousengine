use bevy::{prelude::*, ecs::system::EntityCommands, core_pipeline::tonemapping::{DebandDither, Tonemapping}, render::{view::{ColorGrading, VisibleEntities}, camera::CameraRenderGraph, primitives::{Frustum, CascadesFrusta, CubemapFrusta}}, pbr::{CascadeShadowConfig, Cascades, CascadesVisibleEntities, CubemapVisibleEntities}};
use json::JsonValue;
use nebulousengine_utils::{*, optionals::*, enums::*};

pub fn spawn_entity_from_path(
    commands: &mut Commands, 
    path: &str, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>,
    visible: bool
) {
    spawn_entity_from_json(commands, &load_file_to_json(path), asset_server, meshes, materials, position_offset, rotation_offset, scale_mult, visible);
}

pub fn spawn_entity_from_json(
    commands: &mut Commands, 
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position_offset: Option<Vec3>,
    rotation_offset: Option<Quat>,
    scale_mult: Option<Vec3>,
    visible: bool
) {
    // create entity
    let mut entity = commands.spawn_empty();

    // unpack json
    let components = &input_json["components"];

    // if components is an array, loop through each component
    if components.is_array() {
        for i in 0 .. components.len() {
            // add each component to the entity
            let bundle = unpack_component(&components[i], asset_server, meshes, materials);
            if bundle.is_ok() {
                bundle.unwrap().attach(&mut entity);
            } else {
                warn!("Failed to convert json to component with input {}", input_json)
            }
        }
    }

    // add visibility
    entity.insert(
        if visible { Visibility::Inherited }
        else { Visibility::Hidden }
    ).insert(ComputedVisibility::default());
    
    // add transform
    let mut transform = optional_transform(input_json, "transform");
    if position_offset.is_some() { transform.translation += position_offset.unwrap() }
    if rotation_offset.is_some() { transform.rotate(rotation_offset.unwrap()) }
    if scale_mult.is_some() { transform.scale *= scale_mult.unwrap() }
    entity.insert(transform).insert(GlobalTransform::default());
}

pub enum EntityBundle {
    Model(Handle<Scene>),
    Camera(((Camera, Projection, Tonemapping, DebandDither, ColorGrading), UiCameraConfig, Option<MainCamera>)),
    DirectionalLight(DirectionalLight),
    PointLight(PointLight),
    SpotLight(SpotLight),
    Shape((Handle<Mesh>, Handle<StandardMaterial>))
}

impl EntityBundle {
    fn attach(self, commands: &mut EntityCommands) {
        match self {
            Self::Model(bundle) => commands.insert(bundle),
            Self::Camera((camera, ui_config, main_camera)) => {
                // insert not optional camera components
                commands.insert(camera).insert(ui_config)
                    .insert(CameraRenderGraph::new("core_3d")).insert(VisibleEntities::default())
                    .insert(Frustum::default()).insert(Camera3d::default());

                // add main camera if able
                if main_camera.is_some() { commands.insert(main_camera.unwrap()); }

                // return the commands to match others
                commands
            },
            Self::DirectionalLight(bundle) => commands.insert(bundle)
                .insert(CascadesFrusta::default()).insert(Cascades::default())
                .insert(CascadeShadowConfig::default()).insert(CascadesVisibleEntities::default()),
            Self::PointLight(bundle) => commands.insert(bundle)
                .insert(CubemapVisibleEntities::default()).insert(CubemapFrusta::default()),
            Self::SpotLight(bundle) => commands.insert(bundle)
                .insert(VisibleEntities::default()).insert(Frustum::default()),
            Self::Shape(bundle) => commands.insert(bundle),
        };
    }
}

fn unpack_component(
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>
) -> Result<EntityBundle, String> {
    // unpack json
    let type_str = input_json["type"].as_str();

    // make sure unpacked info is correct
    if type_str.is_none() {
        return Err(format!("Could not grab type from component"));
    }
    let type_str = type_str.unwrap();

    // match the component and add it to the entity
    Ok(match type_str {
        "model" => {
            EntityBundle::Model(
                asset_server.load(format!("{}#Scene0", optional_string(input_json, "model")).as_str())
            )
        },
        "camera" => EntityBundle::Camera((
            (
                Camera {
                    viewport: optional_viewport(input_json, "viewport"),
                    order: optional_isize(input_json, "order", 0),
                    is_active: optional_bool(input_json, "active", true),
                    hdr: optional_bool(input_json, "hdr", false), // WARN EXPERIMENTAL
                    msaa_writeback: optional_bool(input_json, "msaa_writeback", true), // WARN EXPERIMENTAL
                    ..Default::default()
                },
                projection(optional_string(input_json, "projection")),
                tonemapping(optional_string(input_json, "tonemapping")),
                optional_deband_dither(input_json, "dither"),
                optional_color_grading(input_json, "color_grading")
            ),
            UiCameraConfig {
                show_ui: optional_bool(input_json, "show_ui", true)
            },
            if optional_bool(input_json, "main", false) { Some(MainCamera) } else { None }
        )),
        "directional_light" => EntityBundle::DirectionalLight(
            DirectionalLight {
                color: optional_color_default(input_json, "color", Color::WHITE),
                illuminance: optional_f32(input_json, "intensity", 10000.0),
                shadows_enabled: optional_bool(input_json, "shadows_enabled", true),
                shadow_depth_bias: optional_f32(input_json, "shadow_depth_bias", 0.02),
                shadow_normal_bias: optional_f32(input_json, "shadow_normal_bias", 0.6)
            }
        ),
        "point_light" => EntityBundle::PointLight(
            PointLight {
                color: optional_color_default(input_json, "color", Color::WHITE),
                intensity: optional_f32(input_json, "intensity", 800.0),
                range: optional_f32(input_json, "range", 20.0),
                radius: optional_f32(input_json, "radius", 0.0),
                shadows_enabled: optional_bool(input_json, "shadows_enabled", true),
                shadow_depth_bias: optional_f32(input_json, "shadow_depth_bias", 0.02),
                shadow_normal_bias: optional_f32(input_json, "shadow_normal_bias", 0.6)
            }
        ),
        "shape" => EntityBundle::Shape((
            meshes.add(unpack_shape(input_json)),
            materials.add(optional_color_default(input_json, "color", Color::WHITE).into())
        )),
        _ => return Err(format!("Could not add type {}", type_str))
    })
}

fn unpack_shape(json: &JsonValue) -> Mesh {
    return match optional_string(json, "shape") {
        "box" => {
            if json.has_key("from") && json.has_key("to") {
                shape::Box::from_corners(
                    optional_vec3(json, "from", Vec3::ZERO), 
                    optional_vec3(json, "to", Vec3::ZERO)
                ).into()
            } else {
                shape::Box::new(
                    optional_f32(json, "width", 0.0),
                    optional_f32(json, "height", 0.0),
                    optional_f32(json, "depth", 0.0),
                ).into()
            }
        },
        "capsule" => shape::Capsule {
            radius: optional_f32(json, "radius", 0.5),
            rings: optional_usize(json, "rings", 0),
            depth: optional_f32(json, "depth", 1.0),
            latitudes: optional_usize(json, "latitudes", 16),
            longitudes: optional_usize(json, "longitudes", 32),
            uv_profile: capsule_uv_mapping(optional_string(json, "uv_profile"))
        }.into(),
        "circle" => shape::Circle {
            radius: optional_f32(json, "radius", 0.5),
            vertices: optional_usize(json, "vertices", 64)
        }.into(),
        "cube" => shape::Cube {
            size: optional_f32(json, "size", 1.0)
        }.into(),
        "cylinder" => shape::Cylinder {
            radius: optional_f32(json, "radius", 0.5),
            height: optional_f32(json, "height", 1.0),
            resolution: optional_u32(json, "resolution", 16),
            segments: optional_u32(json, "segments", 1)
        }.into(),
        "ico_sphere" => shape::Icosphere {
            radius: optional_f32(json, "radius", 1.0),
            subdivisions: optional_usize(json, "subdivisions", 5)
        }.try_into().unwrap(),
        "plane" => shape::Plane::from_size(optional_f32(json, "size", 1.0)).into(),
        "quad" => if optional_bool(json, "flip", false) {
            shape::Quad::new(optional_vec2(json, "size"))
        } else { shape::Quad::flipped(optional_vec2(json, "size")) }.into(),
        "polygon" => shape::RegularPolygon::new(
            optional_f32(json, "radius", 1.0),
            optional_usize(json, "sides", 3)
        ).into(),
        "torus" => shape::Torus {
            radius: optional_f32(json, "radius", 1.0),
            ring_radius: optional_f32(json, "ring_radius", 0.5),
            subdivisions_segments: optional_usize(json, "subdivisions_segments", 32),
            subdivisions_sides: optional_usize(json, "subdivisions_sides", 24)
        }.into(),
        "sphere" => shape::UVSphere {
            radius: optional_f32(json, "radius", 1.0),
            sectors: optional_usize(json, "sectors", 36),
            stacks: optional_usize(json, "stacks", 18)
        }.into(),
        _ => shape::Plane::from_size(0.0).into()
    }
}
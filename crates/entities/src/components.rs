use bevy::{prelude::*, ecs::system::EntityCommands, core_pipeline::tonemapping::{DebandDither, Tonemapping}, render::{view::{ColorGrading, VisibleEntities}, camera::CameraRenderGraph, primitives::{Frustum, CascadesFrusta, CubemapFrusta}}, pbr::{CascadeShadowConfig, Cascades, CascadesVisibleEntities, CubemapVisibleEntities}};
use bevy_rapier3d::prelude::{RigidBody, Collider, Restitution, Sensor, Friction, ColliderMassProperties, MassProperties, KinematicCharacterController, CharacterLength, CharacterAutostep};
use json::JsonValue;
use nebulousengine_utils::{*, optionals::*, enums::*};

// enum wrapper for components
pub enum EntityBundle {
    Model(Handle<Scene>),
    Camera(((Camera, Projection, Tonemapping, DebandDither, ColorGrading), UiCameraConfig, Option<MainCamera>)),
    DirectionalLight(DirectionalLight),
    PointLight(PointLight),
    SpotLight(SpotLight),
    Shape((Handle<Mesh>, Handle<StandardMaterial>)),
    Rigidbody(RigidBody),
    Collider((Collider, Option<Sensor>, Option<ColliderMassProperties>, Option<Friction>)),
    Elasticity(Restitution),
    Friction(Friction),
    CharacterController(KinematicCharacterController)
}

impl EntityBundle {
    pub fn attach(self, commands: &mut EntityCommands) {
        // attach each component and its corresponding defaults to the given entity
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
            Self::Rigidbody(bundle) => commands.insert(bundle),
            Self::Collider((collider, sensor, mass_props, friction)) => {
                commands.insert(collider);

                if sensor.is_some() {
                    commands.insert(sensor.unwrap());
                }

                if mass_props.is_some() {
                    commands.insert(mass_props.unwrap());
                }

                if friction.is_some() {
                    commands.insert(friction.unwrap());
                }

                commands
            },
            Self::Elasticity(bundle) => commands.insert(bundle),
            Self::Friction(bundle) => commands.insert(bundle),
            Self::CharacterController(bundle) => commands.insert(bundle)
        };
    }
}

pub fn unpack_component(
    input_json: &JsonValue, 
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>, 
    no_cam_spawn: bool
) -> Result<EntityBundle, String> {
    // unpack json
    let type_str = input_json["type"].as_str();

    // make sure unpacked info is correct
    if type_str.is_none() {
        return Err(format!("Could not grab type from component"));
    }
    let type_str = type_str.unwrap();

    // match the component and add it to the entity
    match type_str {
        "model" => Ok(EntityBundle::Model(
            asset_server.load(format!("{}#Scene0", optional_string(input_json, "model")).as_str())
        )),
        "camera" => if no_cam_spawn {
            Err(format!("No Camera Spawn"))
        } else {
                Ok(EntityBundle::Camera((
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
            )))
        },
        "directional_light" => Ok(EntityBundle::DirectionalLight(
            DirectionalLight {
                color: optional_color_default(input_json, "color", Color::WHITE),
                illuminance: optional_f32(input_json, "illuminance", 10000.0),
                shadows_enabled: optional_bool(input_json, "shadows_enabled", true),
                shadow_depth_bias: optional_f32(input_json, "shadow_depth_bias", 0.02),
                shadow_normal_bias: optional_f32(input_json, "shadow_normal_bias", 0.6)
            }
        )),
        "point_light" => Ok(EntityBundle::PointLight(
            PointLight {
                color: optional_color_default(input_json, "color", Color::WHITE),
                intensity: optional_f32(input_json, "intensity", 800.0),
                range: optional_f32(input_json, "range", 20.0),
                radius: optional_f32(input_json, "radius", 0.0),
                shadows_enabled: optional_bool(input_json, "shadows_enabled", true),
                shadow_depth_bias: optional_f32(input_json, "shadow_depth_bias", 0.02),
                shadow_normal_bias: optional_f32(input_json, "shadow_normal_bias", 0.6)
            }
        )),
        "shape" => Ok(EntityBundle::Shape((
            meshes.add(unpack_shape(input_json)),
            materials.add(optional_color_default(input_json, "color", Color::WHITE).into())
        ))),
        "elasticity" => Ok(EntityBundle::Elasticity(Restitution::coefficient(optional_f32(input_json, "elasticity", 0.0)))),
        "collider" => {
            let collider = unpack_collider(input_json);

            if collider.is_ok() {
                let collider = collider.unwrap();
                let is_sensor = optional_bool(input_json, "is_sensor", false);
                let is_sensor = if is_sensor { Some(Sensor) } else { None };
                let mass_props = collider_mass_properties(input_json);
                let mass_props = if mass_props.is_ok() { Some(mass_props.unwrap()) } else { None };
                let friction = input_json["friction"].as_f32();
                let friction = if friction.is_some() { Some(Friction::coefficient(friction.unwrap())) } else { None };
                Ok(EntityBundle::Collider((collider, is_sensor, mass_props, friction)))
            } else {
                Err(collider.err().unwrap())
            }
        },
        "rigidbody" => Ok(EntityBundle::Rigidbody(rigidbody(optional_string(input_json, "physics_type")))),
        "character_controller" => {
            // build offset
            let offset = character_length(input_json, "offset", "offset_type", 0.01);

            // build autostep
            let autostep = &input_json["autostep"];
            let autostep = if autostep.is_object() {
                Some(CharacterAutostep {
                    max_height: character_length(autostep, "max_height", "max_height_length_type", 0.25),
                    min_width: character_length(autostep, "min_width", "min_width_length_type", 0.1),
                    include_dynamic_bodies: optional_bool(autostep, "include_dynamic_bodies", true)
                })
            } else { None };

            Ok(EntityBundle::CharacterController(KinematicCharacterController { 
                translation: if input_json.has_key("translation") { Some(optional_vec3(input_json, "translation", Vec3::ZERO)) } else { None }, 
                custom_mass: if input_json.has_key("mass") { Some(optional_f32(input_json, "mass", 0.0)) } else { None }, 
                up: optional_vec3(input_json, "up", Vec3::Y),
                offset: offset,
                slide: optional_bool(input_json, "slide", false),
                autostep: autostep,
                max_slope_climb_angle: optional_f32(input_json, "max_climb_angle", 45.0).to_radians(),
                min_slope_slide_angle: optional_f32(input_json, "min_slide_angle", 30.0).to_radians(),
                apply_impulse_to_dynamic_bodies: optional_bool(input_json, "move_dynamic_bodies", true),
                snap_to_ground: if input_json.has_key("snap_to_ground") { Some(character_length(input_json, "snap_to_ground", "snap_to_ground_length_type", 0.2)) } else { None },
                ..Default::default()
            }))
        },
        _ => return Err(format!("Could not add type {}", type_str))
    }
}

fn unpack_collider(json: &JsonValue) -> Result<Collider, String> {
    let shape_str = &json["shape"];

    if shape_str.is_string() {
        match shape_str.as_str().unwrap() {
            "sphere" => Ok(Collider::ball(optional_f32(json, "radius", 1.0))),
            "cylinder" => Ok(Collider::cylinder(
                optional_f32(json, "height", 1.0) / 2.0, 
                optional_f32(json, "radius", 1.0)
            )),
            "rounded_cylinder" => Ok(Collider::round_cylinder(
                optional_f32(json, "height", 1.0) / 2.0, 
                optional_f32(json, "radius", 1.0), 
                optional_f32(json, "border_radius", 0.0)
            )),
            "cone" => Ok(Collider::cone(
                optional_f32(json, "height", 1.0) / 2.0, 
                optional_f32(json, "radius", 1.0)
            )),
            "rounded_cone" => Ok(Collider::round_cone(
                optional_f32(json, "height", 1.0) / 2.0, 
                optional_f32(json, "radius", 1.0), 
                optional_f32(json, "border_radius", 0.0)
            )),
            "capsule" => {
                let axis = &json["axis"].as_str();

                if axis.is_some() {
                    match axis.unwrap() {
                        "x" => Ok(Collider::capsule_x(
                            optional_f32(json, "height", 1.0) / 2.0, 
                            optional_f32(json, "radius", 1.0)
                        )),
                        "y" => Ok(Collider::capsule_y(
                            optional_f32(json, "height", 1.0) / 2.0, 
                            optional_f32(json, "radius", 1.0)
                        )),
                        "z" => Ok(Collider::capsule_z(
                            optional_f32(json, "height", 1.0) / 2.0, 
                            optional_f32(json, "radius", 1.0)
                        )),
                        "free" => Ok(Collider::capsule(
                            optional_vec3(json, "point_a", Vec3::ZERO), 
                            optional_vec3(json, "point_b", Vec3::ZERO), 
                            optional_f32(json, "radius", 1.0)
                        )),
                        _ => Err(format!("Unknown axis {}", axis.unwrap()))
                    }
                } else {
                    Ok(Collider::capsule_y(
                        optional_f32(json, "height", 1.0) / 2.0, 
                        optional_f32(json, "radius", 1.0)
                    ))
                }
            },
            "cube" => Ok(Collider::cuboid(
                optional_f32(json, "width", 1.0), 
                optional_f32(json, "height", 1.0), 
                optional_f32(json, "depth", 1.0)
            )),
            "rounded_cube" => Ok(Collider::round_cuboid(
                optional_f32(json, "width", 1.0), 
                optional_f32(json, "height", 1.0), 
                optional_f32(json, "depth", 1.0),
                optional_f32(json, "radius", 1.0)
            )),
            "triangle" => Ok(Collider::triangle(
                optional_vec3(json, "point_a", Vec3::ZERO), 
                optional_vec3(json, "point_b", Vec3::ZERO), 
                optional_vec3(json, "point_c", Vec3::ZERO)
            )),
            "rounded_triangle" => Ok(Collider::round_triangle(
                optional_vec3(json, "point_a", Vec3::ZERO), 
                optional_vec3(json, "point_b", Vec3::ZERO), 
                optional_vec3(json, "point_c", Vec3::ZERO),
                optional_f32(json, "radius", 1.0)
            )),
            _ => Err(format!("No shape with id {} in collider", shape_str))
        }
    } else {
        Err("No shape id was given in collider".to_string())
    }
}

fn rigidbody(target: &str) -> RigidBody {
    match target {
        "dynamic" => RigidBody::Dynamic,
        "fixed" => RigidBody::Fixed,
        "kinematic_position" => RigidBody::KinematicPositionBased,
        "kinematic_velocity" => RigidBody::KinematicVelocityBased,
        _ => RigidBody::default()
    }
}

fn collider_mass_properties(json: &JsonValue) -> Result<ColliderMassProperties, String> {
    if json.has_key("center_of_mass") {
        Ok(ColliderMassProperties::MassProperties(MassProperties {
            local_center_of_mass: optional_vec3(json, "center_of_mass", Vec3::ZERO),
            mass: optional_f32(json, "mass", 0.0),
            principal_inertia_local_frame: optional_quat(json, "inertia_local_frame", Quat::default()),
            principal_inertia: optional_vec3(json, "inertia", Vec3::ZERO)
        }))
    } else if json.has_key("mass") {
        Ok(ColliderMassProperties::Mass(
            optional_f32(json, "mass", 0.0)
        ))
    } else if json.has_key("density") {
        Ok(ColliderMassProperties::Density(
            optional_f32(json, "density", 0.0)
        ))
    } else {
        Err("Could not convert to collider mass properties".to_string())
    }
}

fn character_length(json: &JsonValue, value_name: &str, type_name: &str, default_value: f32) -> CharacterLength {
    let offset_val = optional_f32(json, value_name, default_value);
    let offset_type = json[type_name].as_str().unwrap_or("");
    match offset_type {
        "relative" => CharacterLength::Relative(offset_val),
        _ => CharacterLength::Absolute(offset_val)
    }
}

fn unpack_shape(json: &JsonValue) -> Mesh {
    // match the shape string to a shape and create it accordingly
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
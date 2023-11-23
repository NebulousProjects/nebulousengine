use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_models::{gltf::GLTFModel, animator::component::Animator};
use nebulousengine_utils::state_machines::asset::{StateMachine, StateMachineElement, StateMachineNext, StateMachineCondition, StateMachineTrigger};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut statemachines: ResMut<Assets<StateMachine>>
) {
    // create statemachine for the fox
    let statemachine = statemachines.add(
        StateMachine { 
            default: "Survey".to_string(), 
            elements: vec![
                StateMachineElement {
                    name: "Survey".to_string(),
                    next: vec![
                        StateMachineNext {
                            name: "Run".to_string(),
                            condition: StateMachineCondition::None,
                            priority: 0.0
                        }
                    ],
                    triggers: vec![
                        StateMachineTrigger {
                            trigger: "start".to_string(),
                            priority: 0.0
                        }
                    ]
                },
                StateMachineElement {
                    name: "Run".to_string(),
                    next: vec![
                        StateMachineNext {
                            name: "Walk".to_string(),
                            condition: StateMachineCondition::None,
                            priority: 0.0
                        }
                    ],
                    triggers: vec![]
                },
                StateMachineElement {
                    name: "Walk".to_string(),
                    next: vec![
                        StateMachineNext {
                            name: "Survey".to_string(),
                            condition: StateMachineCondition::None,
                            priority: 0.0
                        }
                    ],
                    triggers: vec![]
                }
            ]
        }
    );

    // create fox
    commands.spawn((
        TransformBundle::default(),
        VisibilityBundle::default(),
        GLTFModel::from_str(&asset_server, "fox.glb", Vec3::ZERO, Vec3 { x: 0.025, y: 0.025, z: 0.025 }),
        Animator {
            handle: statemachine,
            ..Default::default()
        }
    ));

    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // camera
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

fn update(
    mut fox: Query<&mut Transform, With<Animator>>,
    time: Res<Time>
) {
    let mut transform = fox.single_mut();
    transform.rotate_y(time.delta_seconds());
}

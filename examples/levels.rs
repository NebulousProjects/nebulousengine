use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_levels::{Levels, levels::Level, CurrentLevel};

#[derive(States, Default, Debug, Clone, Hash, Eq, PartialEq)]
enum TestLevels {
    #[default]
    TestA,
    TestB
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine, Levels::<TestLevels>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .add_systems(OnEnter(TestLevels::TestA), start_a)
        .add_systems(OnEnter(TestLevels::TestB), start_b)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }); 
}

fn update(
    mut levels: ResMut<Level<TestLevels>>,
    key_inputs: Res<Input<KeyCode>>
) {     
    if key_inputs.just_released(KeyCode::Space) {
        levels.goto(TestLevels::TestB);
    } else if key_inputs.just_released(KeyCode::B) {
        levels.goto(TestLevels::TestA);
    }
}

fn start_a(
    level: ResMut<Level<TestLevels>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let a = level.spawn(&mut commands, 
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }
    );
    a.insert(VisibilityBundle::default());
}

fn start_b(
    level: ResMut<Level<TestLevels>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    level.spawn(&mut commands, 
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb_u8(255, 144, 124).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        }
    );
}

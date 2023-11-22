use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_ui::{camera::UICamera, node::UINode};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ui: ResMut<UINode>
) {
    commands.spawn(Camera2dBundle::default());

    ui.panel()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .bg(Color::RED)
        .children(|ui| {
            ui.panel().id("camera_view")
                .width(Val::Percent(50.0))
                .height(Val::Percent(50.0))
                .top(Val::Percent(25.0))
                .left(Val::Percent(25.0));
        });

    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into()),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        UiCameraConfig { show_ui: false },
        UICamera::new("camera_view".to_string())
    ));
}
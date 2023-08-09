use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_ui::component::{Ui, UiCommand, UiCommandType, UiBundle};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ui
    commands.spawn(UiBundle {
        ui: Ui::from_handle(asset_server.load("test.ui")),
        ..Default::default()
    });
    // commands.spawn((
    //     TransformBundle::default(),
    //     VisibilityBundle::default(),
    //     Ui::from_handle(asset_server.load("test.ui"))
    // ));
}

fn update(
    time: Res<Time>,
    mut ui: Query<&mut Ui>
) {
    let ui = ui.iter_mut().next();
    let mut ui = if ui.is_some() { ui.unwrap() } else { return };
    ui.commands.push(UiCommand { 
        target: "fps".to_string(), 
        command: UiCommandType::ModText { 
            new_text: Text::from_section(
                format!("FPS: {}", 1. / time.delta_seconds()), 
                TextStyle { 
                    font_size: 25.0, 
                    color: Color::WHITE, 
                    ..Default::default() 
                }
            )
        } 
    });
}

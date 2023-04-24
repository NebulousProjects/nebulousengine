use std::f32::consts::PI;

use bevy::{prelude::*, render::render_resource::*};

use nebulousengine_entities::spawn_entity_from_path;
use nebulousengine_scripting::*;
use nebulousengine_utils::*;
use nebulousengine_ui::*;
// use nebulousengine_ui::convert_uifile_to_uibundle;
// use nebulousengine_editor::EditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         title: "Nebulous Engine Game".into(), // todo make this dynamic
        //         present_mode: PresentMode::AutoNoVsync,
                
        //         ..default()
        //     }),
        //     ..default()
        // }))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(EditorPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ScriptingPlugin)
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

#[derive(Component)]
struct Shape;

const X_EXTENT: f32 = 14.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     ..default()
    // });

    // let shapes = [
    //     meshes.add(shape::Cube::default().into()),
    //     meshes.add(shape::Box::default().into()),
    //     meshes.add(shape::Capsule::default().into()),
    //     meshes.add(shape::Torus::default().into()),
    //     meshes.add(shape::Cylinder::default().into()),
    //     meshes.add(shape::Icosphere::default().try_into().unwrap()),
    //     meshes.add(shape::UVSphere::default().into()),
    // ];

    // let num_shapes = shapes.len();

    // for (i, shape) in shapes.into_iter().enumerate() {
    //     commands.spawn((
    //         PbrBundle {
    //             mesh: shape,
    //             material: debug_material.clone(),
    //             transform: Transform::from_xyz(
    //                 -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
    //                 2.0,
    //                 0.0,
    //             )
    //             .with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //             ..default()
    //         },
    //         Shape,
    //     ));
    // }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    }, UiCameraConfig {
        show_ui: true,
    }, MainCamera));

    // ui
    add_ui_json_to_commands(&load_file_to_json("./assets/test.ui"), &mut commands, &asset_server);

    // entities
    spawn_entity_from_path(&mut commands, "./assets/test.entity", &asset_server);
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
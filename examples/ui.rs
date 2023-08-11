use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_ui::{component::*, serializables::*};

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
) {

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

    let mut scroll_ui = Ui::from_handle(asset_server.load("test_scroll.ui"));
    for i in 0 .. 30 {
        scroll_ui.add_element("list", UiElement {
            subtype: UiElementType::Text,
            text: format!("Item {}", i + 1),
            font_size: 25.,
            color: Color::BLACK,
            ..Default::default()
        });
    }
    commands.spawn(UiBundle {
        ui: scroll_ui,
        ..Default::default()
    });

    commands.spawn(UiBundle {
        ui: Ui::from_handle(asset_server.load("test_collapsable.ui")),
        ..Default::default()
    });
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
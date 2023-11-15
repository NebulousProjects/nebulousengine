use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_ui::{node::UINode, ui::UI, events::UIEvents};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut ui: ResMut<UINode>
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // draw ui
    ui.text("FPS").id("FPS_Text");
    
    ui
        .button(Some(Color::BLUE), Some(Color::GREEN))
        .style(Style {
            position_type: PositionType::Absolute,
            top:   Val::Px(0.0),
            right: Val::Px(500.0),
            padding: UiRect::all(Val::Px(5.0)),
            width: Val::Px(200.0),
            ..Default::default()
        })
        .bg(Color::RED)
        .id("Test_Button")
        .children(|ui| {
            ui.text("Click me!");
        });

    ui.scroll_panel().id("Scrolling List")
        .bg(Color::GRAY)
        .style(Style {
            position_type: PositionType::Absolute,
            top:    Val::Percent(25.0),
            right:  Val::Px(20.0),
            height: Val::Percent(50.0),
            ..Default::default()
        })
        .children(|ui| {
            for i in 0 .. 30 {
                ui.text(format!("Text {}", i));
            }
        });
}

fn update(
    time: Res<Time>,
    mut ui: ResMut<UINode>,
    ui_events: Res<UIEvents>
    // test: Query<&TestComponent>
) {
    // update fps counter
    ui.get_mut("FPS_Text").unwrap().ui(UI::Text { text: format!("FPS: {}", 1. / time.delta_seconds()) });

    if ui_events.just_pressed("Test_Button") { println!("Test button pressed!") }
}
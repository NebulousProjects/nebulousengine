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
        .button().id("Test Button")
        .hover_color(Color::BLUE, None)
        .press_color(Color::GREEN, None)
        .border(UiRect::all(Val::Px(5.0)), Color::BLACK)
        .bg(Color::RED)
        .position_type(PositionType::Absolute)
        .top(Val::Px(0.0))
        .right(Val::Px(500.0))
        .width(Val::Px(200.0))
        .padding(UiRect::all(Val::Px(5.0)))
        .children(|ui| {
            ui.text("Click me!");
        });

    ui.scroll_panel(FlexDirection::Column).id("Scrolling List")
        .bg(Color::GRAY)
        .position_type(PositionType::Absolute)
        .top(Val::Percent(25.0))
        .right(Val::Px(20.0))
        .height(Val::Percent(50.0))
        .children(|ui| {
            for i in 0 .. 30 {
                ui.text(format!("Text {}", i));
            }
        });

    ui.panel()
        .image("image.png").id("Test Image")
        .position_type(PositionType::Absolute)
        .width(Val::Percent(20.0))
        .height(Val::Percent(20.0))
        .bottom(Val::Px(0.0))
        .left(Val::Px(0.0));

    ui.slider(FlexDirection::Row, Color::GREEN, Color::RED, 0.3)
        .id("Test Slider")
        .border(UiRect::all(Val::Px(5.0)), Color::BLACK)
        .position_type(PositionType::Absolute)
        .bottom(Val::Px(50.0))
        .left(Val::Percent(30.0))
        .width(Val::Percent(20.0))
        .height(Val::Px(20.0))
        .bg(Color::WHITE)
        .moveable(true)
        .children(|ui| {
            ui.button()
                .width(Val::Px(40.0))
                .height(Val::Px(40.0))
                .bg(Color::PURPLE)
                .border(UiRect::all(Val::Px(5.0)), Color::BLACK);
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
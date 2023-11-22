use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_ui::node::UINode;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut ui: ResMut<UINode>
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // main panel
    ui.panel()
    .width(Val::Percent(100.0))
    .height(Val::Percent(100.0))
    .flex_direction(FlexDirection::Column)
    .children(|ui| {

        // title
        ui.text_area(Color::BLACK, 25.0)
            .ghost_text("Enter text...")
            .selected_border(Color::BLACK)
            .width(Val::Percent(100.0))
            .height(Val::Percent(8.0))
            .bg(Color::WHITE)
            .border(UiRect::all(Val::Px(5.0)), Color::GRAY);

        // main box
        ui.text_area(Color::BLACK, 20.0)
            .ghost_text("Enter text...")
            .selected_border(Color::BLACK)
            .width(Val::Percent(100.0))
            .height(Val::Percent(84.0))
            .bg(Color::WHITE)
            .border(UiRect::all(Val::Px(5.0)), Color::GRAY);

        // commit button
        ui.button()
            .width(Val::Percent(100.0))
            .height(Val::Percent(8.0))
            .border(UiRect::all(Val::Px(5.0)), Color::GRAY)
            .bg(Color::TOMATO)
            .justify_content(JustifyContent::Center)
            .text("Commit");
    });
}
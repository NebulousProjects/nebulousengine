use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_ui::{node::UINode, ui::UI};

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
        ui.add(UI::TextArea {
            selected_bg: None,
            selected_border: Some(Color::BLACK),
            font_size: 25.0,
            multiline: false,
        }).width(Val::Percent(100.0))
            .height(Val::Percent(8.0))
            .bg(Color::WHITE)
            .border(UiRect::all(Val::Px(5.0)), Color::GRAY);

        // main box
        ui.add(UI::TextArea {
            selected_bg: None,
            selected_border: Some(Color::BLACK),
            font_size: 15.0,
            multiline: false,
        }).width(Val::Percent(100.0))
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
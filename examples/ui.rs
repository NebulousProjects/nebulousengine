use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_ui::{node::UINode, ui::UI};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, NebulousEngine))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut ui: ResMut<UINode>
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // draw ui
    ui.panel().id("FPS_Panel")
        .style(Style {
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..Default::default()
        })
        .children(|ui| {
            ui.text("FPS").id("FPS_Text");
        });
    
    ui.panel().id("Button_Panel")
        .style(Style {
            align_items: AlignItems::Start,
            justify_content: JustifyContent::End,
            ..Default::default()
        })
        .children(|ui| {
            ui
                .button(Some(Color::BLUE), Some(Color::GREEN))
                .style(Style {
                    padding: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                })
                .bg(Color::RED)
                .id("Test_Button")
                .children(|ui| {
                    ui.text("Click me!");
                });
        });
}

fn update(
    time: Res<Time>,
    mut ui: ResMut<UINode>
    // test: Query<&TestComponent>
) {
    // update fps counter
    ui.get_mut("FPS_Text").unwrap().ui(UI::Text { text: format!("FPS: {}", 1. / time.delta_seconds()) });

    // let ui = ui.iter_mut().next();
    // let mut ui = if ui.is_some() { ui.unwrap() } else { return };
    // ui.commands.push(UiCommand { 
    //     target: "fps".to_string(), 
    //     command: UiCommandType::ModText { 
    //         new_text: Text::from_section(
    //             format!("FPS: {}", 1. / time.delta_seconds()), 
    //             TextStyle { 
    //                 font_size: 25.0, 
    //                 color: Color::WHITE, 
    //                 ..Default::default() 
    //             }
    //         )
    //     } 
    // });
}
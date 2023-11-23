use bevy::prelude::*;
use nebulousengine::NebulousEngine;
use nebulousengine_input::structs::*;
use nebulousengine_ui::node::*;

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
    commands.spawn(Camera2dBundle::default());

    // create input description
    let description = InputDescription::create(|inputs| {
        inputs.insert("vertical", vec![
            InputType::AXIS { positive: InputElement::Keyboard { key: KeyCode::W }, negative: InputElement::Keyboard { key: KeyCode::S } },
            InputType::SCALAR { element: InputElement::GamepadAxis { axis: GamepadAxisType::LeftStickY, mult: 1.0 } }
        ]);
        inputs.insert("horizontal", vec![
            InputType::AXIS { positive: InputElement::Keyboard { key: KeyCode::D }, negative: InputElement::Keyboard { key: KeyCode::A } },
            InputType::SCALAR { element: InputElement::GamepadAxis { axis: GamepadAxisType::LeftStickX, mult: 1.0 } }
        ]);
        inputs.insert("click", vec![
            InputType::SCALAR { element: InputElement::Mouse { button: MouseButton::Left } },
            InputType::SCALAR { element: InputElement::GamepadButton { button: GamepadButtonType::North } }
        ]);
    });

    // create ui
    let panel = ui.scroll_panel(FlexDirection::Column)
        .width(Val::Percent(100.0)).height(Val::Percent(100.0));
    description.elements.iter().for_each(|(name, _)| {
        panel.panel()
            .flex_direction(FlexDirection::Row)
            .width(Val::Percent(100.0))
            .children(|ui| {
                ui.text(name);
                ui.text("0.0").id(name).margin(UiRect::left(Val::Percent(5.0)));
                // ui.slider(FlexDirection::Row, Color::GREEN, Color::RED, 0.0).id(name);
            });
    });

    // spawn input description
    commands.spawn(Inputs::from_description(description));
}

fn update(
    input: Query<&Inputs>,
    mut ui: ResMut<UINode>
) {
    let input = input.get_single();
    let input = if input.is_ok() { input.unwrap() } else { return };
    input.values.iter().for_each(|(name, value)| {
        ui.get_mut(name).unwrap().set_text(format!("{}", value));
    });
}

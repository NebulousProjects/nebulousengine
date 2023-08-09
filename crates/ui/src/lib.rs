use bevy::{prelude::*, ecs::system::EntityCommands};
use loader::UiLoader;
use serde_json::Value;
use serializables::*;
use component::*;

pub mod serializables;
pub mod component;
mod loader;

#[derive(Component)]
pub struct SpawnedUi;

#[derive(Component)]
pub struct UiID(String);

#[derive(Component)]
pub struct UiData(Value);

#[derive(Event)]
pub struct UiPressed(pub Entity, pub String, pub Option<Value>);
#[derive(Event)]
pub struct UiHoverStart(pub Entity, pub String, pub Option<Value>);
#[derive(Event)]
pub struct UiReset(pub Entity, pub String, pub Option<Value>);

// plugin for uis
pub struct ConfigurableUiPlugin;
impl Plugin for ConfigurableUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<UiElement>()
            .add_event::<UiPressed>()
            .add_event::<UiHoverStart>()
            .add_event::<UiReset>()
            .add_asset_loader(UiLoader)
            .add_systems(Update, (spawn_uis, handle_buttons, handle_commands));
    }
}

// spawn all uis
fn spawn_uis(
    mut commands: Commands,
    entities: Query<(Entity, Option<&Ui>, Option<&Handle<UiElement>>), (Without<SpawnedUi>, Or<(&Ui, &Handle<UiElement>)>)>,
    uis: Res<Assets<UiElement>>
) {
    // loop through all uis to spawn
    entities.for_each(|(entity, ui, handle)| {
        // get ui element from spawnable
        let element = if handle.is_some() {
            let handle = uis.get(handle.unwrap());
            if handle.is_none() { return }
            Some(handle.unwrap())
        } else {
            match &ui.unwrap().spawnable {
                UiSpawnable::Handle { handle } => {
                    let handle = uis.get(handle);
                    if handle.is_none() { return }
                    Some(handle.unwrap())
                },
                UiSpawnable::Direct { element } => Some(element),
                UiSpawnable::Empty => None
            }
        };

        // spawn ui
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(SpawnedUi);
        if element.is_some() {
            attach_ui(element.unwrap(), &mut entity_commands);
        }
    });
}

fn attach_ui(
    element: &UiElement,
    commands: &mut EntityCommands
) {
    // add bundle to element
    element.insert_bundle(commands);

    // add children
    commands.with_children(|builder| {
        element.children.iter().for_each(|element| {
            let mut entity_commands = builder.spawn_empty();
            attach_ui(element, &mut entity_commands);
        });
    });
}

fn handle_buttons(
    buttons_changed: Query<(Entity, &Interaction, &UiID, Option<&UiData>), (With<Button>, Changed<Interaction>)>,
    mut pressed_events: EventWriter<UiPressed>,
    mut hover_start_events: EventWriter<UiHoverStart>,
    mut reset_events: EventWriter<UiReset>
) {
    // if a button is pressed, send event for each type of interaction
    buttons_changed.for_each(|(entity, interaction, id, data)| {
        // handle data
        let data = match data {
            None => None,
            Some(data) => Some(data.0.clone())
        };

        // broadcast event
        match *interaction {
            Interaction::Pressed => pressed_events.send(UiPressed(entity, id.0.clone(), data)),
            Interaction::Hovered => hover_start_events.send(UiHoverStart(entity, id.0.clone(), data)),
            Interaction::None => reset_events.send(UiReset(entity, id.0.clone(), data))
        }
    });
}

fn handle_commands(
    mut commands: Commands,
    mut ui_assets: ResMut<Assets<UiElement>>,
    mut uis: Query<&mut Ui, With<SpawnedUi>>,
    mut ui_elements: Query<(Entity, &UiID, Option<&mut Text>, Option<&mut BackgroundColor>, Option<&mut BorderColor>)>
) {
    // for each ui
    uis.for_each_mut(|mut ui| {
        // if no commands stop here
        if ui.commands.is_empty() { return }

        // handle commands
        ui.commands.iter_mut().for_each(|command| {
            // get target
            let target = ui_elements.iter_mut().find(|(_, id, _, _, _)| id.0 == command.target);
            let (target, _, text, bg_color, border_color) = if target.is_some() { target.unwrap() } else { return };

            // process command
            match &mut command.command {
                UiCommandType::Add { spawnable } => {
                    // get handle to ui element
                    let handle = match spawnable {
                        UiSpawnable::Handle { handle } => handle.clone(),
                        UiSpawnable::Direct { element } => ui_assets.add(element.clone()),
                        UiSpawnable::Empty => return
                    };

                    // add ui
                    commands.entity(target).remove::<SpawnedUi>().insert(handle);
                },
                UiCommandType::Remove => commands.entity(target).despawn_recursive(),
                UiCommandType::ModText { new_text } => {
                    if text.is_some() {
                        let mut text = text.unwrap();
                        text.as_reflect_mut().apply(new_text.as_reflect_mut());
                    }
                },
                UiCommandType::ModBGColor { color } => {
                    if bg_color.is_some() {
                        let mut bg_color = bg_color.unwrap();
                        bg_color.0 = color.clone();
                    }
                },
                UiCommandType::ModBorderColor { color } => {
                    if border_color.is_some() {
                        let mut border_color = border_color.unwrap();
                        border_color.0 = color.clone();
                    }
                },
            }
        });

        // clear commands
        ui.commands.clear();
    });
}

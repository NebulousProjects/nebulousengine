use bevy::{prelude::*, ecs::system::EntityCommands, input::mouse::{MouseWheel, MouseScrollUnit}, window::PrimaryWindow, math::Vec3Swizzles};
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
pub struct UiID(pub String);

#[derive(Component)]
pub struct UiData(pub Value);

#[derive(Event)]
pub struct UiPressed(pub Entity, pub String, pub Option<Value>);
#[derive(Event)]
pub struct UiHoverStart(pub Entity, pub String, pub Option<Value>);
#[derive(Event)]
pub struct UiReset(pub Entity, pub String, pub Option<Value>);
#[derive(Event)]
pub struct UiCollapsibleOpen(pub Entity, pub Option<String>, pub Option<Value>);
#[derive(Event)]
pub struct UiCollapsibleClose(pub Entity, pub Option<String>, pub Option<Value>);

// plugin for uis
pub struct ConfigurableUiPlugin;
impl Plugin for ConfigurableUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<UiElement>()
            .add_event::<UiPressed>()
            .add_event::<UiHoverStart>()
            .add_event::<UiReset>()
            .add_event::<UiCollapsibleOpen>()
            .add_event::<UiCollapsibleClose>()
            .add_asset_loader(UiLoader)
            .add_systems(Update, (spawn_uis, handle_buttons, handle_commands, update_scrolling_lists, update_collapsible));
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
    buttons_changed: Query<(Entity, &Interaction, &UiID, Option<&UiData>), (With<Button>, Changed<Interaction>, Without<Collapsible>)>,
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
                    // add ui
                    commands.entity(target).with_children(|parent| {
                        let spawnable = spawnable.clone();
                        parent.spawn((UiBundle {
                            ui: Ui::from_spawnable(spawnable),
                            ..Default::default()
                        }, Node::default(), Style::default()));
                    });
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

fn update_scrolling_lists(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<(&mut ScrollList, &mut Style, &GlobalTransform, &Parent, &Node)>,
    nodes: Query<&Node>,
    window: Query<&Window, With<PrimaryWindow>>
) {
    // get window
    let window = window.get_single();
    let window = if window.is_ok() { window.unwrap() } else { return };
    let pointer = window.cursor_position();
    let pointer = if pointer.is_some() { pointer.unwrap() } else { return };

    // handle mouse wheel eve nt
    for event in mouse_wheel.iter() {
        query.for_each_mut(|(mut scroll, mut style, transform, parent, node)| {
            // get the heights of this element and of its children
            let items_height = node.size().y;
            let container_size = nodes.get(parent.get()).unwrap().size();
            let container_height = container_size.y;

            // if this element is not hovered, cancel
            let node_position = transform.translation().xy();
            let half_size = 0.5 * container_size;
            let min = node_position - half_size;
            let max = node_position + half_size;
            if !((min.x .. max.x).contains(&pointer.x) && (min.y .. max.y).contains(&pointer.y)) { return }

            // get maximum scroll distance
            let max_scroll = (items_height - container_height).max(0.);

            // get the change in the y access
            let dy = match event.unit {
                MouseScrollUnit::Line => event.y * 20.,
                MouseScrollUnit::Pixel => event.y,
            };

            // apply scroll
            scroll.amount += dy;
            scroll.amount = scroll.amount.clamp(-max_scroll, 0.);
            style.top = Val::Px(scroll.amount);
        });
    }
}

fn update_collapsible(
    mut collapsibles: Query<(Entity, &Interaction, Option<&UiID>, Option<&UiData>, &mut Collapsible, &Children), (With<Button>, Changed<Interaction>, With<Collapsible>)>,
    mut possible_children: Query<&mut Visibility, (Without<NoCollapse>, With<Style>)>,
    mut open_events: EventWriter<UiCollapsibleOpen>,
    mut close_events: EventWriter<UiCollapsibleClose>
) {
    // for each collapsible that changed
    collapsibles.for_each_mut(|(entity, interaction, id, data, mut collapsible, children)| {
        // make sure pressed
        if !matches!(interaction, Interaction::Pressed) { return }

        // get target visibility
        let target_visibility = collapsible.collapsed;
        collapsible.collapsed = !target_visibility;
        let new_visibility = match target_visibility {
            true => Visibility::Inherited,
            false => Visibility::Hidden
        };

        // update visiblity of children
        children.iter().for_each(|child| {
            let visibility = possible_children.get_mut(*child);
            if visibility.is_ok() { 
                let mut visibility = visibility.unwrap();
                visibility.as_reflect_mut().apply(new_visibility.as_reflect());
            }
        });

        // map id to optional string
        let id = match id {
            Some(_) => Some(id.unwrap().0.clone()),
            None => None,
        };

        // map data
        let data = match data {
            Some(_) => Some(data.unwrap().0.clone()),
            None => None
        };

        // broadcast event
        match target_visibility {
            true => open_events.send(UiCollapsibleOpen(entity, id, data)),
            false => close_events.send(UiCollapsibleClose(entity, id, data))
        }
    });
}

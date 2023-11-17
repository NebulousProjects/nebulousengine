use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}, window::PrimaryWindow};
use events::*;
use node::UINode;
use ui::render_ui;

pub mod events;
pub mod node;
pub mod ui;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct OriginalColor(pub Color, pub Option<Color>);
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct HoverColor(pub Color, pub Option<Color>);
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct PressColor(pub Color, pub Option<Color>);

#[derive(Component, Default, Debug, Clone)]
pub struct UIID(String);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UIScrollList { pub position: f32 }

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UISlider;

// plugin for uis
pub struct ConfigurableUIPlugin;
impl Plugin for ConfigurableUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UIEventsPlugin)
            .init_resource::<UINode>()
            .add_systems(Update, (update_ui, update_hover_press, update_scroll));
    }
}

fn update_ui(
    mut commands: Commands,
    mut ui: ResMut<UINode>,
    mut asset_server: ResMut<AssetServer>
) {
    // if no root representation, create one and stop, otherwise, return entity reference
    let entity = if ui.representation.is_none() {
        let entity = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        });
        ui.representation = Some(entity.id());
        return;
    } else { ui.representation.as_ref().unwrap().clone() };

    // check if each child should render
    ui.children.iter_mut().for_each(|child| {
        check_should_render(&mut commands, &mut asset_server, &entity, child);
    });
}

fn check_should_render(commands: &mut Commands, asset_server: &mut ResMut<AssetServer>, parent: &Entity, ui: &mut UINode) {
    // if should render, remove old representation and render
    if ui.is_dirty || ui.representation.is_none() {
        // remove old representation
        if ui.representation.is_some() {
            commands.entity(ui.representation.unwrap()).despawn_recursive();
        }

        // call render
        commands.entity(*parent).with_children(|builder| {
            render_ui(asset_server, builder, ui);
        });
    } 
    // otherwise, check if children need to render
    else if ui.ui.do_children_render_check() {
        ui.children.iter_mut().for_each(|child| {
            check_should_render(commands, asset_server, ui.representation.as_ref().unwrap(), child);
        });
    }
}

fn update_hover_press(
    mut query: Query<(&mut BackgroundColor, &OriginalColor, Option<&HoverColor>, Option<&PressColor>, &Interaction), Changed<Interaction>>
) {
    // for each button interaction, update background color
    query.for_each_mut(|(
        mut bg, original, 
        hover, press, interaction
    )| {
        match interaction {
            Interaction::Pressed => if press.is_some() { *bg = press.unwrap().0.into() },
            Interaction::Hovered => if hover.is_some() { *bg = hover.unwrap().0.into() },
            Interaction::None => *bg = original.0.into(),
        }
    });
}

fn update_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut UIScrollList, &mut Style, &Parent, &Node)>,
    nodes: Query<(&Node, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>
) {
    // get delta x from mouse wheel events
    let mut dy = 0.0;
    for event in mouse_wheel_events.read() {
        dy += match event.unit {
            MouseScrollUnit::Line => event.y * 20.,
            MouseScrollUnit::Pixel => event.y,
        };
    }

    // get mouse position
    let mouse_position = window.single().cursor_position().unwrap_or(Vec2 { x: 0.0, y: 0.0 });

    // for each in scroll list
    for (mut list, mut style, parent, list_node) in &mut query {
        // get parent node and transform
        let (parent_node, parent_transform) = nodes.get(parent.get()).unwrap();

        // make sure mouse position is inside parent
        let node_position = parent_transform.translation().xy();
        let half_size = 0.5 * parent_node.size();
        let min = node_position - half_size;
        let max = node_position + half_size;
        if !(min.x .. max.x).contains(&mouse_position.x) || 
            !(min.y .. max.y).contains(&mouse_position.y) { return }

        // get maximum scroll distance
        let items_height = list_node.size().y;
        let container_height = parent_node.size().y;
        let max_scroll = (items_height - container_height).max(0.);

        // update and clamp scroll
        list.position += dy;
        list.position = list.position.clamp(-max_scroll, 0.);
        style.top = Val::Px(list.position);
    }
}

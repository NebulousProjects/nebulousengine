use bevy::{prelude::*, input::mouse::{MouseWheel, MouseScrollUnit}, window::PrimaryWindow};
use events::*;
use node::UINode;
use ui::{render_ui, UI};

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

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UISliderFirst;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UISliderSecond;

#[derive(Component, Default, Debug, Clone)]
pub struct UITextArea {
    current: String,
    cursor_position: usize,
    multiline: bool
}

// plugin for uis
pub struct ConfigurableUIPlugin;
impl Plugin for ConfigurableUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UIEventsPlugin)
            .init_resource::<UINode>()
            .add_systems(Update, (update_ui, update_hover_press, update_scroll, update_sliders));
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

fn update_sliders(
    mut ui: ResMut<UINode>,
    window: Query<&Window, With<PrimaryWindow>>,
    sliders: Query<(&Node, &GlobalTransform, &Style, &UIID, &Children), With<UISlider>>,
    mut buttons: Query<(&Node, &mut Style, Option<&Interaction>, Option<&UISliderFirst>, Option<&UISliderSecond>), Without<UISlider>>,

) {
    // get mouse position
    let window = window.single();
    let mouse_position = window.cursor_position().unwrap_or(Vec2 { x: 0.0, y: 0.0 });

    // update all sliders
    sliders.for_each(|(slider, slider_transform, slider_style, slider_id, children)| {
        // get slider info
        let info = ui.get_mut(slider_id.0.clone());
        let info = if info.is_none() { return } else { info.unwrap() };
        let (direction, current_amount, moveable) = match &info.ui {
            ui::UI::Slider { direction, amount, moveable, .. } => (*direction, *amount, *moveable),
            _ => return
        };

        // calculate border width and height
        let border_width = slider_style.border.left.resolve(slider.size().x, slider.size()).unwrap_or(0.0) +
            slider_style.border.right.resolve(slider.size().x, slider.size()).unwrap_or(0.0);
        let border_height = slider_style.border.top.resolve(slider.size().y, slider.size()).unwrap_or(0.0) +
            slider_style.border.bottom.resolve(slider.size().y, slider.size()).unwrap_or(0.0);

        // update children and if any are pressed, allow amount changes
        let mut allow_changes = false;
        children.iter().for_each(|child| {
            // unpack child
            let button = buttons.get_mut(*child);
            if button.is_err() { return }
            let (button, mut style, interaction, first, second) = button.unwrap();

            // if has interaction and interaction is pressed, allow changes
            if interaction.unwrap_or(&Interaction::None) == &Interaction::Pressed { allow_changes = true; }

            // update any moveable buttons, center on amount point
            if interaction.is_some() {
                match direction {
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        style.position_type = PositionType::Absolute;
                        style.top = Val::Px((slider.size().y - button.size().y) * current_amount - (border_height / 2.0));
                        style.left = Val::Px((button.size().x + border_width - slider.size().x) / -2.0);
                    },
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        style.position_type = PositionType::Absolute;
                        style.top = Val::Px((button.size().y + border_height - slider.size().y) / -2.0);
                        style.left = Val::Px((slider.size().x - button.size().x) * current_amount - (border_width / 2.0));
                    }
                }
            } 
            // update first (left side) slider part
            else if first.is_some() {
                match direction {
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        style.width = Val::Percent(100.0);
                        style.height = Val::Percent(current_amount * 100.0);
                    },
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        style.width = Val::Percent(current_amount * 100.0);
                        style.height = Val::Percent(100.0);
                    }
                }
            } 
            // update second (right side) slider part
            else if second.is_some() {
                match direction {
                    FlexDirection::Column | FlexDirection::ColumnReverse => {
                        style.width = Val::Percent(100.0);
                        style.height = Val::Percent((1.0 - current_amount) * 100.0);
                    },
                    FlexDirection::Row | FlexDirection::RowReverse => {
                        style.width = Val::Percent((1.0 - current_amount) * 100.0);
                        style.height = Val::Percent(100.0);
                    }
                }
            }
        });

        // if changes allowed
        if allow_changes && moveable {
            // get amount the mouse position would represent
            let dist = slider_transform.translation().x - mouse_position.x;
            let amount = (-dist / slider.size().x + 0.5).clamp(0.0, 1.0);
            
            // update ui if possible
            match &info.ui {
                ui::UI::Slider { direction, first, second, moveable, .. } => {
                    info.ui = UI::Slider { direction: *direction, first: *first, second: *second, amount, moveable: *moveable };
                },
                _ => {}
            }
        }
    });
}

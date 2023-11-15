use bevy::prelude::*;
use events::*;
use node::UINode;
use ui::render_ui;

pub mod events;
pub mod node;
pub mod ui;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct OriginalColor(Color);
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct HoverColor(Color);
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct PressColor(Color);

#[derive(Component, Default, Debug, Clone)]
pub struct UIID(String);

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UIScrollList { pub position: f32 }

// plugin for uis
pub struct ConfigurableUIPlugin;
impl Plugin for ConfigurableUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UIEventsPlugin)
            .init_resource::<UINode>()
            .add_systems(Update, (update_ui, update_hover_press));
    }
}

fn update_ui(
    mut commands: Commands,
    mut ui: ResMut<UINode>
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
        check_should_render(&mut commands, &entity, child);
    });
}

fn check_should_render(commands: &mut Commands, parent: &Entity, ui: &mut UINode) {
    // if should render, remove old representation and render
    if ui.is_dirty || ui.representation.is_none() {
        // remove old representation
        if ui.representation.is_some() {
            commands.entity(ui.representation.unwrap()).despawn_recursive();
        }

        // call render
        commands.entity(*parent).with_children(|builder| {
            render_ui(builder, ui);
        });
    } 
    // otherwise, check if children need to render
    else {
        ui.children.iter_mut().for_each(|child| {
            check_should_render(commands, ui.representation.as_ref().unwrap(), child);
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

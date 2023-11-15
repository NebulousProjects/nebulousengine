use bevy::prelude::*;

use crate::{node::UINode, OriginalColor, HoverColor, PressColor, UIID, UIScrollList};

#[derive(Resource, Default, Debug, Clone)]
pub enum UI {
    #[default]
    Panel,
    ScrollPanel,
    Text {
        text: String
    },
    Button {
        hover_bg: Option<Color>,
        press_bg: Option<Color>
    }
}

pub fn render_ui(commands: &mut ChildBuilder, ui: &mut UINode) {
    // setup style
    let mut style = ui.style.clone();

    // render
    let mut entity = match &ui.ui {
        UI::Panel => {
            // spawn node
            let mut spawned = commands.spawn(NodeBundle { 
                style, 
                background_color: BackgroundColor(ui.background_color), 
                ..Default::default() 
            });
            
            // add children
            spawned.with_children(|builder| {
                ui.children.iter_mut().for_each(|child| {
                    render_ui(builder, child);
                });
            });

            spawned
        },
        UI::ScrollPanel => {
            // update style
            style.overflow = Overflow::clip_y();
            style.flex_direction = FlexDirection::Column;

            // spawn node
            let mut spawned = commands.spawn(NodeBundle { 
                style, 
                background_color: BackgroundColor(ui.background_color), 
                ..Default::default() 
            });
            
            // add children
            spawned.with_children(|builder| {
                builder.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        top: Val::Px(-200.0),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(UIScrollList::default()).with_children(|builder| {
                    ui.children.iter_mut().for_each(|child| {
                        render_ui(builder, child);
                    });
                });
            });

            spawned
        },
        UI::Text { text } => {
            // spawn text
            let mut spawned = commands.spawn(TextBundle {
                text: Text::from_section(text, TextStyle { color: Color::WHITE, font_size: 25.0, ..Default::default() }),
                style,
                background_color: BackgroundColor(ui.background_color),
                ..Default::default()
            });
            
            // add children
            spawned.with_children(|builder| {
                ui.children.iter_mut().for_each(|child| {
                    render_ui(builder, child);
                });
            });

            spawned
        },
        UI::Button { hover_bg, press_bg } => {
            // spawn button
            let mut spawned = commands.spawn((
                ButtonBundle {
                    style,
                    background_color: BackgroundColor(ui.background_color),
                    ..Default::default()
                },
                OriginalColor(ui.background_color)
            ));

            // add hover and press colors
            if hover_bg.is_some() { spawned.insert(HoverColor(hover_bg.unwrap())); }
            if press_bg.is_some() { spawned.insert(PressColor(press_bg.unwrap())); }
            
            // add children
            spawned.with_children(|builder| {
                ui.children.iter_mut().for_each(|child| {
                    render_ui(builder, child);
                });
            });

            spawned
        }
    };

    // add id
    if ui.id.is_some() {
        entity.insert(UIID(ui.id.clone().unwrap()));
    }

    // update ui node
    let id = entity.id();
    ui.representation = Some(id);
    ui.is_dirty = false;
}

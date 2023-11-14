use bevy::prelude::*;

use crate::{node::UINode, OriginalColor, HoverColor, PressColor};

#[derive(Resource, Default, Debug, Clone)]
pub enum UI {
    #[default]
    Panel,
    Text {
        text: String
    },
    Button {
        hover_bg: Option<Color>,
        press_bg: Option<Color>
    }
}

pub fn render_ui(commands: &mut ChildBuilder, ui: &mut UINode, is_root: bool) {
    // setup style
    let mut style = ui.style.clone();
    if is_root {
        style.position_type = PositionType::Absolute;
        style.width = Val::Percent(100.);
        style.height = Val::Percent(100.);
    }

    // render
    let mut entity = match &ui.ui {
        UI::Panel => commands.spawn(NodeBundle { 
            style, 
            background_color: BackgroundColor(ui.background_color), 
            ..Default::default() 
        }),
        UI::Text { text } => commands.spawn(TextBundle {
            text: Text::from_section(text, TextStyle { color: Color::WHITE, font_size: 25.0, ..Default::default() }),
            style,
            background_color: BackgroundColor(ui.background_color),
            ..Default::default()
        }),
        UI::Button { hover_bg, press_bg } => {
            let mut spawned = commands.spawn((
                ButtonBundle {
                    style,
                    background_color: BackgroundColor(ui.background_color),
                    ..Default::default()
                },
                OriginalColor(ui.background_color)
            ));
            if hover_bg.is_some() { spawned.insert(HoverColor(hover_bg.unwrap())); }
            if press_bg.is_some() { spawned.insert(PressColor(press_bg.unwrap())); }
            spawned
        },
    };

    // render children
    entity.with_children(|builder| {
        ui.children.iter_mut().for_each(|child| {
            render_ui(builder, child, false);
        });
    });

    // update ui node
    let id = entity.id();
    ui.representation = Some(id);
    ui.is_dirty = false;
}

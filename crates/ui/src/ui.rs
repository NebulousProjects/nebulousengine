use bevy::prelude::*;

use crate::{node::UINode, OriginalColor, HoverColor, PressColor, UIID, UIScrollList};

#[derive(Resource, Default, Debug, Clone)]
pub enum UI {
    #[default]
    Panel,
    ScrollPanel { flex_direction: FlexDirection },
    Text { text: String },
    Button {
        hover_bg: Option<HoverColor>,
        press_bg: Option<PressColor>
    },
    Slider { direction: FlexDirection, first: Color, second: Color, amount: f32, moveable: bool }
}

pub fn render_ui(asset_server: &mut ResMut<AssetServer>, commands: &mut ChildBuilder, ui: &mut UINode) {
    // setup style
    let mut style = ui.style.clone();
    if ui.border.is_some() {
        style.border = ui.border.unwrap().0;
    }

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
                    render_ui(asset_server, builder, child);
                });
            });

            spawned
        },
        UI::ScrollPanel { flex_direction } => {
            // update style
            style.flex_direction = *flex_direction;
            style.overflow = match flex_direction {
                FlexDirection::Row => Overflow::clip_x(),
                FlexDirection::Column => Overflow::clip_y(),
                FlexDirection::RowReverse => Overflow::clip_x(),
                FlexDirection::ColumnReverse => Overflow::clip_y(),
            };

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
                        flex_direction: *flex_direction,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(UIScrollList::default()).with_children(|builder| {
                    ui.children.iter_mut().for_each(|child| {
                        render_ui(asset_server, builder, child);
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
                    render_ui(asset_server, builder, child);
                });
            });

            spawned
        },
        UI::Button { hover_bg, press_bg } => {
            // get border color
            let border_color = if ui.border.is_some() { Some(ui.border.unwrap().1) } else { None };

            // spawn button
            let mut spawned = commands.spawn((
                ButtonBundle {
                    style,
                    background_color: BackgroundColor(ui.background_color),
                    ..Default::default()
                },
                OriginalColor(ui.background_color, border_color)
            ));

            // add hover and press colors
            if hover_bg.is_some() { spawned.insert(hover_bg.unwrap()); }
            if press_bg.is_some() { spawned.insert(press_bg.unwrap()); }
            
            // add children
            spawned.with_children(|builder| {
                ui.children.iter_mut().for_each(|child| {
                    render_ui(asset_server, builder, child);
                });
            });

            spawned
        }
        UI::Slider { direction, first, second, amount, moveable } => {
            // make sure width and height are something
            style.flex_direction = *direction;

            // sort width and heights by direction
            let left_size = Val::Percent(*amount * 100.0);
            let right_size = Val::Percent((1.0 - *amount) * 100.0);
            let (left_width, left_height, right_width, right_height) = match direction {
                FlexDirection::Column | FlexDirection::ColumnReverse => 
                (Val::Percent(100.0), left_size, Val::Percent(100.0), right_size),
                FlexDirection::Row    | FlexDirection::RowReverse    => 
                (left_size, Val::Percent(100.0), right_size, Val::Percent(100.0))
            };

            // spawn root
            let mut spawned = commands.spawn(NodeBundle { 
                style, 
                background_color: BackgroundColor(ui.background_color), 
                ..Default::default() 
            });
            
            spawned.with_children(|builder| {
                // add left and right displays
                builder.spawn(NodeBundle {
                    style: Style {
                        width: left_width,
                        height: left_height,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(*first),
                    ..Default::default()
                });
                builder.spawn(NodeBundle {
                    style: Style {
                        width: right_width,
                        height: right_height,
                        ..Default::default()
                    },
                    background_color: BackgroundColor(*second),
                    ..Default::default()
                });

                // add children normally if not moveable
                println!("Moveable: {}", moveable);
                // if !moveable {
                //     ui.children.iter_mut().for_each(|child| {
                //         render_ui(asset_server, builder, child);
                //     });
                // }
            });

            spawned
        }
    };

    // give border color
    if ui.border.is_some() {
        entity.insert(BorderColor(ui.border.unwrap().1));
    }

    // give image
    if ui.image.is_some() {
        entity.insert(UiImage::new(asset_server.load(ui.image.as_ref().unwrap())));
    }

    // add id
    if ui.id.is_some() {
        entity.insert(UIID(ui.id.clone().unwrap()));
    }

    // update ui node
    let id = entity.id();
    ui.representation = Some(id);
    ui.is_dirty = false;
}

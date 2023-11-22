use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};

use crate::{OriginalColor, events::UIEvents, UIID};

#[derive(Component, Default, Debug, Clone)]
pub struct UITextArea {
    pub current: String,
    pub ghost_text: String,
    pub cursor_position: usize,
    pub selected_bg: Option<Color>,
    pub selected_border: Option<Color>,
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UITextAreaText;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UITextAreaSelected;

// text area plugin
pub struct UITextAreaPlugin;
impl Plugin for UITextAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_text_areas, do_typing, render));
    }
}

fn select_text_areas(
    mut commands: Commands,
    mut area_buttons: Query<(Entity, &mut BackgroundColor, &mut BorderColor, &UITextArea, &Interaction), (Changed<Interaction>, With<UITextArea>, Without<UITextAreaSelected>)>,
    mut selected: Query<(Entity, &mut BackgroundColor, &mut BorderColor, &OriginalColor), With<UITextAreaSelected>>
) {
    // get new area
    let new_area = area_buttons.get_single_mut();
    let (new_area, mut background, mut border, text_area, interaction) = if new_area.is_ok() { new_area.unwrap() } else { return };

    // only if interaction is pressed
    if !matches!(interaction, &Interaction::Pressed) { return }

    // remove old selected
    selected.for_each_mut(|(entity, mut background, mut border, original)| {
        commands.entity(entity).remove::<UITextAreaSelected>();
        background.0 = original.0;
        if original.1.is_some() { border.0 = original.1.unwrap(); }
    });

    // mark selected
    commands.entity(new_area).insert(UITextAreaSelected);
    if text_area.selected_bg.is_some() { background.0 = text_area.selected_bg.unwrap(); }
    if text_area.selected_border.is_some() { border.0 = text_area.selected_border.unwrap(); }
}

fn do_typing(
    mut events: ResMut<UIEvents>,
    mut selection: Query<(&mut UITextArea, Option<&UIID>), With<UITextAreaSelected>>,
    mut typing: EventReader<ReceivedCharacter>,
    mut keys: EventReader<KeyboardInput>
) {
    // get selection
    let selection = selection.get_single_mut();
    let (mut selection, id) = if selection.is_ok() { selection.unwrap() } else { return };

    // update from keyboard inputs
    keys.read().for_each(|event| {
        if !matches!(event.state, ButtonState::Pressed) { return }
        let key_code = event.key_code;
        let key_code = if key_code.is_some() { key_code.unwrap() } else { return };
        match key_code {
            // move cursor left
            KeyCode::Left => {
                if selection.cursor_position > 0 {
                    selection.cursor_position -= 1;
                }
            }

            // move cursor right
            KeyCode::Right => {
                if selection.cursor_position < selection.current.len() {
                    selection.cursor_position += 1;
                }
            }
            
            _ => {}
        }
    });

    // update text from typing
    typing.read().for_each(|event| {
        // apply char based on what it is
        match event.char {
            '\u{8}' => { // backspace
                if selection.cursor_position > 0 {
                    // selection.current.pop();
                    let cursor = selection.cursor_position - 1;
                    selection.current.remove(cursor);
                    selection.cursor_position -= 1;
                }
            },
            '\r' => { // enter
                // if selection.multiline {
                //     let cursor = selection.cursor_position;
                //     selection.current.insert(cursor, '\n');
                //     selection.cursor_position += 1;
                // }
            }
            _ => {
                let cursor = selection.cursor_position;
                selection.current.insert(cursor, event.char);
                selection.cursor_position += 1;
            }
        }

        // if this text area has an id, update text input in events
        if id.is_some() {
            events.update_text_input(id.unwrap().0.clone(), selection.current.clone());
        }
    });
}

fn render(
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    selection: Query<(&mut UITextArea, &Children, Option<&UITextAreaSelected>)>,
    mut text: Query<&mut Text>,
) {
    selection.for_each(|(selection, children, selected)| {
        // split current string
        let is_selected = selected.is_some();

        // update text in children
        children.iter().for_each(|child| {
            // get text
            let text = text.get_mut(*child);
            let mut text = if text.is_ok() { text.unwrap() } else { return };

            // get style
            let style = &text.sections.first().unwrap().style;

            // write section based on if selected
            if is_selected {
                let (first, second) = selection.current.split_at(selection.cursor_position);
                text.sections = vec![
                    TextSection { value: first.to_string(), style: style.clone() },
                    TextSection { value: "|".to_string(), style: TextStyle { 
                        font: asset_server.load("Cursor.ttf"), 
                        font_size: style.font_size, 
                        color: if time.elapsed_seconds() % 2.0 < 1.0 { style.color } else { Color::NONE } 
                    }},
                    TextSection { value: if selection.current.is_empty() { selection.ghost_text.clone() } else { second.to_string() }, style: style.clone() },
                ];
            } else {
                let draw = if selection.current.is_empty() { selection.ghost_text.clone() } else { selection.current.clone() };
                text.sections = vec![
                    TextSection { value: draw, style: style.clone() }
                ]
            }
        });
    });
}

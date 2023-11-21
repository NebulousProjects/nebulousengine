use bevy::prelude::*;

use crate::OriginalColor;

#[derive(Component, Default, Debug, Clone)]
pub struct UITextArea {
    pub current: String,
    pub cursor_position: usize,
    pub selected_bg: Option<Color>,
    pub selected_border: Option<Color>,
    pub multiline: bool
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UITextAreaText;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct UITextAreaSelected;

// text area plugin
pub struct UITextAreaPlugin;
impl Plugin for UITextAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (select_text_areas, do_typing));
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
    mut selection: Query<(&mut UITextArea, &Children), With<UITextAreaSelected>>,
    mut text: Query<&mut Text>,
    mut typing: EventReader<ReceivedCharacter>
) {
    // get selection
    let selection = selection.get_single_mut();
    let (mut selection, children) = if selection.is_ok() { selection.unwrap() } else { return };

    // update text from typing
    typing.read().for_each(|event| {
        // apply char based on what it is
        match event.char {
            '\u{8}' => {
                if selection.cursor_position > 0 {
                    selection.current.pop();
                    selection.cursor_position -= 1;
                }
            },
            '\r' => {
                if selection.multiline {
                    selection.current.push('\n');
                    selection.cursor_position += 1;
                }
            }
            _ => {
                selection.current.push(event.char);
                selection.cursor_position += 1;
            }
        }

        // update text in children
        children.iter().for_each(|child| {
            let text = text.get_mut(*child);
            let mut text = if text.is_ok() { text.unwrap() } else { return };
            text.sections.iter_mut().for_each(|section| {
                section.value = selection.current.clone();
            });
        });
    });
}

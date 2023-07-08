use bevy::prelude::*;
use bevy_iced::iced::widget::{text, Button, Column};
use bevy_iced::{IcedPlugin, IcedContext};

#[derive(Clone)]
pub enum UiMessage {}

pub struct IcedUIPlugin;
impl Plugin for IcedUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(IcedPlugin)
            .add_event::<UiMessage>()
            .add_system(ui_system);
    }
}

fn ui_system(time: Res<Time>, mut ctx: IcedContext<UiMessage>) {
    ctx.display(
        Column::new()
            .spacing(10)
            .push(text(format!(
                "Hello Iced! Running for {:.2} seconds.",
                time.elapsed_seconds()
            )))
            .push(Button::new(text("Request box")))
    );
}
use bevy::{prelude::*, asset::{AssetLoader, LoadedAsset}, ecs::system::EntityCommands};
use serde_json::Value;
use structs::UiElement;

pub mod structs;

#[derive(Component)]
pub struct SpawnedUi;

#[derive(Component)]
pub struct UiID(String);

#[derive(Component)]
pub struct UiData(Value);

#[derive(Event)]
pub struct UiPressed(pub Entity, pub String, pub Option<Value>);
#[derive(Event)]
pub struct UiHoverStart(pub Entity, pub String, pub Option<Value>);
#[derive(Event)]
pub struct UiReset(pub Entity, pub String, pub Option<Value>);

// plugin for uis
pub struct ConfigurableUiPlugin;
impl Plugin for ConfigurableUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_asset::<UiElement>()
            .add_event::<UiPressed>()
            .add_event::<UiHoverStart>()
            .add_event::<UiReset>()
            .add_asset_loader(UiLoader)
            .add_systems(Update, (spawn_uis, handle_buttons));
    }
}

// asset loader to load ui files
pub struct UiLoader;
impl AssetLoader for UiLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // load content
            let content = std::str::from_utf8(bytes);
            if content.is_err() { error!("Failed to load ui json!"); return Err(bevy::asset::Error::msg("Failed to load json")) }
            let content = content.unwrap();
            
            // load description
            let description: Result<UiElement, serde_json::Error> = serde_json::from_str(content);
            if description.is_err() { error!("Failed to load ui description from json"); return Err(bevy::asset::Error::msg("Failed to load description")) }
            
            // load final input map
            load_context.set_default_asset(LoadedAsset::new(description.unwrap()));
            
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ui"]
    }
}

// spawn all uis
fn spawn_uis(
    mut commands: Commands,
    entities: Query<(Entity, &Handle<UiElement>, Without<SpawnedUi>)>,
    uis: Res<Assets<UiElement>>
) {
    // loop through all uis to spawn
    entities.for_each(|(entity, handle, _)| {
        // get ui element
        let ui = uis.get(handle);
        if ui.is_none() { return }
        let ui = ui.unwrap();

        // spawn ui
        let mut entity_commands = commands.entity(entity);
        entity_commands.insert(SpawnedUi);
        attach_ui(ui, &mut entity_commands);
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
    buttons_changed: Query<(Entity, &Interaction, &UiID, Option<&UiData>), (With<Button>, Changed<Interaction>)>,
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
